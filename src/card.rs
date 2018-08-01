use std::str::FromStr;

use lazyf::{SGetter,Lz};

use self::CardType::*;


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

