use std::str::FromStr;

use lazyf::{SGetter,Lz};

use self::CardType::*;
use bracket_parse::Bracket;
use sc_error::ScErr;


#[derive(Debug,PartialEq,Clone)]
pub struct CardKey{//primary key
    name:String,
    kind:CardType,
}

impl CardKey{
    pub fn new(nm:String,kind:CardType)->CardKey{
        CardKey{
            name:nm,
            kind:kind,
        }
    }

    pub fn from_bracket(b:&Bracket)->Result<CardKey,ScErr>{
        match b {
            Bracket::Branch(v)=>{
                match v.len(){
                    2=>Ok(CardKey{name:v[0].match_str().to_string(),
                                    kind:v[1].match_str().parse()?}),
                    _=>Err(ScErr::NoParse("Not enough args".to_string())),
                }
            },
            _=>Err(ScErr::NoParse("Card Key not Bracker::Branch".to_string())),
        }
    }
}




#[derive(Clone,Copy,Debug,PartialEq)]//,EnumFromStr)]
pub enum CardType{
    Goal,
    Role,
    Trait,
    Skill,
    Event,
    Scenario,
}

impl FromStr for CardType{
    type Err = String;
    fn from_str(s:&str)->Result<Self,Self::Err>{
        match &s.to_lowercase() as &str{
            "goal"=>Ok(Goal),
            "role"=>Ok(Role),
            "trait"=>Ok(Trait),
            "skill"=>Ok(Skill),
            "event"=>Ok(Event),
            "scenario"=>Ok(Scenario),
            r=>Err(format!("Not a Card Type : {}",r)),
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
pub struct Card{
    pub name:String,
    pub text:String,
    pub kind:CardType,
    pub cost:u8,
    pub tokens:u8,
}

impl Card{
    pub fn from_lz(lz:&Lz)->Result<Card,String>{
        let kind = match lz.get_t::<CardType>("tp"){
            Some(k)=>k,
            None=>return Err("Kind not found".to_string()),
        };
        Ok(Card{
            name:lz.name.clone(),
            text:lz.get_s_def("tx",""),
            kind:kind,
            cost:lz.get_t_def("Cost",0),
            tokens:0,
        })
    }
}

