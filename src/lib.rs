extern crate card_deck;
extern crate lazyf;
extern crate itertools;
extern crate bracket_parse;
extern crate rand;
//#[macro_use] extern crate macro_attr;
//#[macro_use] extern crate enum_derive;

use std::path::Path;


pub mod supply;
use supply::{Supply,GrowthRow};

pub mod card;
//use card::CardType;

pub mod player;
use player::{Player};

pub mod action;
use action::{Action,PlAction,PlActionType};

pub mod sc_error;
use sc_error::ScErr;

use rand::{Rng,thread_rng};






pub struct Game{
    pub players:Vec<Player>,
    actions:Vec<Action>,
    growth:GrowthRow,
    supply:Supply,
}

impl Game{
    pub fn build()->GameBuilder{
        GameBuilder::new()
    }

    pub fn player_num(&self, name:&str)->Option<usize>{
        self.players.iter().enumerate().find(|(_,p)|p.name==name).map(|(i,_)|i)
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
            _=>{} 
        }
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
}

pub struct GameBuilder{
    nplayers:usize,
    g_row_size:usize,
    player_names:Option<Vec<String>>,
    supply:Option<Supply>,
    history:Option<Vec<Action>>,
    err:Option<ScErr>,
}

impl GameBuilder{
    pub fn new()->GameBuilder{
        GameBuilder{
            g_row_size:3,
            nplayers:4,
            player_names:None,
            supply:None,
            history:None,
            err:None,
        }
    }

    pub fn supply_file<P:AsRef<Path>>(mut self,fname:P)->Self{
        match Supply::load(fname){
            Ok(s)=>self.supply = Some(s),
            Err(e)=>self.err= Some(e),
        }
        self
    }

    pub fn supply(mut self,sp:Supply)->Self{
        self.supply=Some(sp);
        self
    }
    
    pub fn n_players(mut self, n:usize)->Self{
        self.nplayers = n;
        self
    }

    pub fn player_names<IT>(mut self,names:IT)->Self
        where IT:IntoIterator<Item=String>{
        self.player_names = Some(names.into_iter().collect());
        self
    }

    pub fn done(self)->Result<Game,ScErr>{
        if let Some(e)= self.err {
            return Err(e);
        }
        let pnames:Vec<String> = match self.player_names{
            Some(n)=>n,
            None=> (0..self.nplayers).map(|i|format!("P{}",i)).collect(),
        };

        let mut supply = match self.supply{
            Some(sp)=>sp,
            None=>Supply::load("card_data/cards.lz")?,
        };

        supply.shuffle_decks();

        let players:Vec<Player> = pnames.into_iter().map(|pn| Player::new(&pn,&mut supply)).collect();


        Ok(Game{
            players:players,
            actions:self.history.unwrap_or(Vec::new()),
            growth:GrowthRow::new(self.g_row_size,&mut supply),
            supply:supply,
        })
    }

}


#[cfg(test)]
mod test{
    use ::*;
    #[test]
    fn gm_since(){
        let mut gm = Game::build().done().unwrap();

        for i in 0..4 {
            gm.player_action(PlAction::new("P1",PlActionType::Chat(format!("This action {}",i))));
        }
        assert_eq!(gm.players[0].name , "P0");
        assert_eq!(gm.since(0).len(),4);

        assert_eq!(gm.since(3).len(),1);
        assert_eq!(gm.since(4),&[]);
        assert_eq!(gm.since(5),&[]);
        assert_eq!(gm.since(10),&[]);
    }

}
