extern crate card_deck;
extern crate lazyf;
//#[macro_use] extern crate macro_attr;
//#[macro_use] extern crate enum_derive;

pub mod supply;
use supply::{Supply,GrowthRow};

pub mod card;
use card::{Card};

pub mod player;
use player::{Player};

pub mod action;
use action::Action;




pub struct RewardInfo{
    name:String,dice:u8,tokens:u8,card:String
}



pub struct Game{
    players:Vec<Player>,
    actions:Vec<Action>,
    growth:GrowthRow,
    supply:Supply,
}

impl Game{
    pub fn Build()->GameBuilder{
        GameBuilder::new()
    }
}

pub struct GameBuilder{
    nplayers:usize,
    g_row_size:usize,
    player_names:Option<Vec<String>>,
    supply:Option<Supply>,
    history:Option<Vec<Action>>,
}

impl GameBuilder{
    pub fn new()->GameBuilder{
        GameBuilder{
            g_row_size:3,
            nplayers:4,
            player_names:None,
            supply:None,
            history:None,
        }
    }

    pub fn done(mut self)->Game{
        let mut pnames:Vec<String> = match self.player_names{
            Some(n)=>n,
            None=> (0..self.nplayers).map(|i|format!("P{}",i)).collect(),
        };

        let mut supply = match self.supply{
            Some(sp)=>sp,
            None=>Supply::load("card_data/cards.lz").expect("could not load cards"),
        };

        let mut players:Vec<Player> = pnames.into_iter().map(|pn| Player::new(&pn,&mut supply)).collect();


        Game{
            players:players,
            actions:self.history.unwrap_or(Vec::new()),
            growth:GrowthRow::new(self.g_row_size,&mut supply),
            supply:supply,
        }
    }
}

