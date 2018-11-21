use crate::sc_error::ScErr;

use std::str::FromStr;
use lazyf::{SGetter,Lz};
use self::CardType::*;
use bracket_parse::Bracket;
use std::fmt;


#[derive(Debug,PartialEq,Eq,Hash,Clone,Serialize,Deserialize)]
pub struct CardKey{//primary key
    pub name:String,
    pub kind:CardType,
}

impl fmt::Display for CardKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{:?}", self.name, self.kind)
    }
}

impl CardKey{
    pub fn new(nm:&str,kind:CardType)->CardKey{
        CardKey{
            name:nm.to_string(),
            kind:kind,
        }
    }

    pub fn from_lz(lz:&Lz)->Result<CardKey,ScErr>{
        let kind:CardType = lz.get_s("tp")
                    .ok_or(ScErr::NotFound)?
                    .parse()?;

        Ok(CardKey{
            name:lz.name.clone(),
            kind:kind,
        })
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
            _=>Err(ScErr::NoParse("Card Key not Bracket::Branch".to_string())),
        }
    }
}




#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash,Serialize,Deserialize)]//,EnumFromStr)]
pub enum CardType{
    Goal,
    Role,
    Trait,
    Skill,
    Event,
    Scenario,
}


impl FromStr for CardKey{
    type Err = ScErr;
    fn from_str(s:&str)->Result<Self,ScErr>{
        let mut sp = s.split(',');
        let nm = sp.next().expect("Split returned len 0");
        let kn = sp.next().ok_or(ScErr::NoParse("no comma".to_string()))?;

        Ok(CardKey{
            name:nm.to_string(),
            kind:kn.parse()?,
        })
    }
}

impl FromStr for CardType{
    type Err = ScErr;
    fn from_str(s:&str)->Result<Self,Self::Err>{
        match &s.to_lowercase() as &str{
            "goal"=>Ok(Goal),
            "role"=>Ok(Role),
            "trait"=>Ok(Trait),
            "skill"=>Ok(Skill),
            "event"=>Ok(Event),
            "scenario"=>Ok(Scenario),
            _=>Err(ScErr::NoKind(s.to_string())),
        }
    }
}





