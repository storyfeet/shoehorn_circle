extern crate card_deck;
extern crate lazyf;
extern crate itertools;
//#[macro_use] extern crate macro_attr;
//#[macro_use] extern crate enum_derive;

use std::path::Path;


pub mod supply;
use supply::{Supply,GrowthRow};

pub mod card;

pub mod player;
use player::{Player};

pub mod action;
use action::Action;

pub mod sc_error;
use sc_error::ScErr;








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

