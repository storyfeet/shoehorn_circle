extern crate card_deck;
extern crate lazyf;
//#[macro_use] extern crate macro_attr;
//#[macro_use] extern crate enum_derive;

pub mod supply;
use supply::{Supply};

pub mod card;
use card::{Card};

pub mod player;
use player::{Player};



pub struct BasicAction{
    name:String,
    text:String,
}

pub struct RewardInfo{
    name:String,dice:u8,tokens:u8,card:String
}

pub enum Action{
    Chat(BasicAction),
    Do(BasicAction),
    Say(BasicAction),
    Bid(String,u8),
    Roll(String),//winner
    Reward(),
}

pub struct Game{
    players:Vec<Player>,
    actions:Vec<Action>,
    growth:GrowthRow,
    supply:Supply,
}

