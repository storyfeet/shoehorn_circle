extern crate card_deck;
extern crate lazyf;
extern crate itertools;
extern crate bracket_parse;
extern crate rand;
//#[macro_use] extern crate macro_attr;
//#[macro_use] extern crate enum_derive;

use std::path::Path;


pub mod supply;
use supply::{Supply};

pub mod card;
//use card::CardType;

pub mod player;
use player::{Player};

pub mod action;
use action::{Action,PlAction,PlActionType};

pub mod sc_error;
use sc_error::ScErr;

mod game_builder;
use game_builder::GameBuilder;

use rand::{Rng,thread_rng};



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

    pub fn player_action(&mut self,ac:PlAction){
        use PlActionType::*;
        if self.player_num(&ac.player_name).is_none(){
            return
        }
        
        match ac.act{
            Chat(_)|Do(_)|Say(_)=>{self.actions.push(Action::Pl(ac))},
            Bid(_)=>{
                self.actions.push(Action::Pl(ac));
                self.roll_bids();
            }
            WhoDunnit(s)=>{
                if !self.is_gm(&ac.player_name) {
                    return;
                }
                let dunnit = thread_rng().gen_range(0,self.players.len()+1);
                self.actions.push(Action::WhoDunnitIs(dunnit,s));
            }
            BuyGrowth(s)=>{
                
            }
            _=>{} 
        }
    }
    

    fn run_history(&mut self)->Result<(),ScErr>{
        use action::Action::*;
        for a in &self.actions {
            match a {
                FillGrowth(ref ck)=>{},//TODO};
                _=>{},//TODO
            }

        }
        Ok(())
    }


    fn roll_bids(&mut self){ 
        let mut bids:Vec<Option<u16>> = Vec::new();
        for _ in 0..self.players.len(){
            bids.push(None);
        }
        
        for ac in &self.actions { //Could be more efficient, will do for now
            match ac {
                Action::Pl(PlAction{player_name:ref pname,act:PlActionType::Bid(n)})=>{
                    let pnum = self.player_num(pname).expect("Player name checked already!!!");
                    bids[pnum] = Some(*n as u16);
                },
                Action::Roll(_,_)=>{
                    for b in &mut bids { *b = None; }
                },
                _=>{},
            }
        }

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

        for i in 0..4 {
            gm.player_action(PlAction::new("P1",PlActionType::Chat(format!("This action {}",i))));
        }
        assert_eq!(gm.players[0].name , "P0");
        assert_eq!(gm.since(0).len(),13);//9 growthrow fill actions included

        assert_eq!(gm.since(12).len(),1);
        assert_eq!(gm.since(13),&[]);
        assert_eq!(gm.since(14),&[]);
        assert_eq!(gm.since(20),&[]);
    }

    #[test]
    fn rolls_correct(){
        let mut gm =Game::build().done().unwrap();

        for i in 0 .. 4 {
            gm.player_action(PlAction::new(&pname(i),PlActionType::Bid(2)));
        }
        assert_eq!(gm.actions.len(),14);

        gm.player_action(PlAction::new(&pname(0),PlActionType::Bid(7)));
        assert_eq!(gm.actions.len(),15);
        for i in 1 .. 4 {
            gm.player_action(PlAction::new(&pname(i),PlActionType::Bid(1)));
        }
        assert_eq!(gm.actions.len(),19);

        assert_eq!(gm.curr_gm(),Some("P0"));
    }

}
