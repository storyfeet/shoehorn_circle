extern crate card_deck;
extern crate lazyf;
extern crate itertools;
extern crate bracket_parse;
extern crate rand;
//#[macro_use] extern crate macro_attr;
//#[macro_use] extern crate enum_derive;

pub mod supply;
use supply::{Supply};

pub mod card;
use card::CardKey;

pub mod player;
use player::{Player};

pub mod action;
use action::{Action,Request,RequestType};

pub mod sc_error;
use sc_error::ScErr;

mod game_builder;
use game_builder::GameBuilder;

use rand::{Rng,thread_rng};



#[derive(Debug)]
pub struct Game{
    pub players:Vec<Player>,
    actions:Vec<Action>,
    supply:Supply,
}

impl Game{
    pub fn build()->GameBuilder{
        GameBuilder::new()
    }

    pub fn player_num(&self, name:&str)->Option<usize>{
        self.players.iter().enumerate().find(|(_,p)|p.name==name).map(|(i,_)|i)
    }

    pub fn player<'a>(&'a mut self,name:&str)->Option<&'a mut Player>{
        let pnum = self.player_num(name)?;
        Some(&mut self.players[pnum])
    }

    pub fn player_action(&mut self,ac:Request)->Result<(),ScErr>{
        use RequestType::*;
        let pnum = self.player_num(&ac.player_name).ok_or(ScErr::not_found(&ac.player_name))?;

        match ac.act{
            Chat(s)=>
                self.actions.push(Action::Chat(pnum,s)),
            Do(s)=>
                self.actions.push(Action::Do(pnum,s)),
            Say(s)=>
                self.actions.push(Action::Say(pnum,s)),
            Bid(n)=>{
                self.actions.push(Action::Bid(pnum,n));
                self.roll_bids();
            },
            WhoDunnit(s)=>{
                if !self.is_gm(&ac.player_name) {
                    return Err(ScErr::not_gm(&ac.player_name));
                }
                let dunnit = thread_rng().gen_range(0,self.players.len()+1);
                self.actions.push(Action::WhoDunnitIs(dunnit,s));
            }
            BuyGrowth(buy,token_from)=>{
                let a = self.players[pnum].buy_growth(buy,token_from,&mut self.supply)?;
                self.actions.push(a);
                //TODO, Fill Growth Row, to make up for lost card.

            }
            Reward(pname,ckey,ndice)=>{
                if !self.is_gm(&ac.player_name) {
                    return Err(ScErr::not_gm(&ac.player_name));
                }
                let r_pnum = self.player_num(&pname).ok_or(ScErr::not_found(&pname))?;
                self.players[r_pnum].reward(ckey.clone(),ndice);
                self.actions.push(Action::Reward(r_pnum,ckey,ndice));
                
            }
        };
        Ok(())
    }
    

    fn run_actions<A:IntoIterator<Item=Action>>(&mut self,ac_list:A)->Result<(),ScErr>{
        use action::Action::*;
        for a in ac_list {
            match a {
                AddPlayer(ref pname)=>{
                    let pnum = self.players.len();
                    self.players.push(Player::empty(pname,pnum))
                },
                PlayerDraw(p_ref, ref ckey)=>{
                    let card = self.supply.dig(ckey)?;
                    self.players[p_ref].cards.push(card);
                }
                FillGrowth(ref ck)=>{
                    //TODO
                    
                },
                _=>{},//TODO
            }
            self.actions.push(a);
        }
        Ok(())
    }


    fn roll_bids(&mut self){ 
        let mut bids:Vec<Option<u16>> = Vec::new();
        for _ in 0..self.players.len(){
            bids.push(None);
        }
        
        { //borrow for iterator
            let mut ac_it = (&self.actions).into_iter();
            while let Some(ac)=  ac_it.next_back(){ 
                match ac {
                    Action::Bid(p_ref,n)=>{
                        bids[*p_ref] = Some(*n as u16);
                    },
                    Action::Roll(_,_)=>break,
                    _=>{},
                }
            }
        }//end borrow

        let mut rolls =Vec::new();
        let mut maxn = 0;
        let mut maxp:Option<usize> = None;//None = tie
        while maxp == None {
            rolls= Vec::new();
            for (pn, b) in (&mut bids).into_iter().enumerate(){
                match b{
                    Some(n)=>{
                        let mut r = 0;
                        for _ in 0..*n{
                            r += thread_rng().gen_range(0,6);
                        }
                        if r == maxn {
                            maxp = None;
                        }
                        if r > maxn {
                            maxn = r;
                            maxp = Some(pn);
                        }
                        rolls.push(r);
                    }
                    _=>return,
                }//match
            }//for 
        } //tie
        
        self.actions.push(Action::Roll(self.players[maxp.unwrap()].name.clone(),rolls));
    }

    pub fn since<'a>(&'a self, mut n:usize)->&'a [Action]{
        if n > self.actions.len(){
            n = self.actions.len()
        }
        &self.actions[n..]
    }

    pub fn curr_gm<'a>(&'a self)->Option<&'a str>{
        let mut it = self.actions.iter();
        while let Some(n) =  it.next_back() {
            if let Action::Roll(ref w,_) = n {
                return Some(w)
            }
        }
        None
    }

    pub fn is_gm(&self,nm: &str)->bool{
        match self.curr_gm(){
            Some(s)=>s == nm,
            _=>false,
        }
    }

}




#[cfg(test)]
mod test{
    use ::*;
    
    //test util
    fn pname(n:usize)->String{
        format!("P{}",n)
    }


    #[test]
    fn gm_since(){
        let mut gm = Game::build().done().unwrap();
        let prelen = gm.actions.len();

        for i in 0..4 {
            gm.player_action(Request::new("P1",RequestType::Chat(format!("This action {}",i)))).unwrap();
        }
        assert_eq!(gm.players[0].name , "P0");
        assert_eq!(gm.since(0).len(),prelen + 4);

        assert_eq!(gm.since(prelen).len(),4);
        assert_eq!(gm.since(prelen+4),&[]);
        assert_eq!(gm.since(prelen+5),&[]);
        assert_eq!(gm.since(prelen+20),&[]);
    }

    #[test]
    fn rolls_correct(){
        let mut gm =Game::build().done().unwrap();

        for i in 0 .. 4 {
            gm.player_action(Request::new(&pname(i),RequestType::Bid(2))).unwrap();
        }
        let prelen = gm.actions.len();

        gm.player_action(Request::new(&pname(0),RequestType::Bid(7))).unwrap();
        assert_eq!(gm.actions.len(),prelen+1);
        for i in 1 .. 4 {
            gm.player_action(Request::new(&pname(i),RequestType::Bid(1))).unwrap();
        }
        assert_eq!(gm.actions.len(),prelen + 5);

        assert_eq!(gm.curr_gm(),Some("P0"));
    }


    #[test]
    fn rebuild_history(){
        let gm =Game::build().done().unwrap();

        let history = gm.since(0);

        let gm2 = Game::build().done_history(history.clone().to_vec()).unwrap();

        //TODO add conditions

        assert_eq!( gm2.players.len(),4 );

        for (i,p1) in gm.players.iter().enumerate(){
            let p2= &gm2.players[i]; 
            assert_eq!(p1.name,p2.name);
            assert_eq!(p1.cards,p2.cards);
        }


        
    }


}
