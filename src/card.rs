use std::str::FromStr;

use lazyf::{SGetter,Lz};

use self::CardType::*;
use bracket_parse::Bracket;
use sc_error::ScErr;


#[derive(Debug,PartialEq,Clone)]
pub struct CardKey{//primary key
    pub name:String,
    pub kind:CardType,
}


impl CardKey{
    pub fn new(nm:&str,kind:CardType)->CardKey{
        CardKey{
            name:nm.to_string(),
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
            _=>Err(ScErr::NoParse("Card Key not Bracket::Branch".to_string())),
        }
    }
}

impl<'a> From<&'a Card> for CardKey{
    fn from(c:&Card)->CardKey{
        CardKey::new(&c.name,c.kind)
    }
}


impl PartialEq<Card>for CardKey{
    /// ```
    /// use shoehorn_circle::card::{Card,CardKey,CardType};
    /// let c = Card::new("Pig","some text",CardType::Role,2);
    /// let ck = CardKey::new("Pig",CardType::Role);
    /// assert_eq!(ck,c);
    /// assert_eq!(c,ck);
    /// ```
    fn eq(&self,c:&Card)->bool{
        self.name == c.name && self.kind == c.kind
    }
}
impl PartialEq<CardKey>for Card{
    fn eq(&self,c:&CardKey)->bool{
        self.name == c.name && self.kind == c.kind
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
}

impl Card{
    pub fn new(name:&str,tx:&str,kind:CardType,cost:u8)->Card{
        Card{
            name:name.to_string(),
            text:tx.to_string(),
            kind:kind,
            cost:cost,
        }
    }
    pub fn from_lz(lz:&Lz)->Result<Card,String>{
        let kind = match lz.get_t::<CardType>("tp"){
            Some(k)=>k,
            None=>return Err("Kind not found".to_string()),
        };
        Ok(Card{
            name:lz.name.clone(),
            text:lz.get_s_def("tx",""),
            kind:kind,
            cost:lz.get_t_def("Cost",4),
        })
    }
}

