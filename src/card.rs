use std::str::FromStr;

use lazyf::{SGetter,Lz,LzList,LzErr};

use self::CardType::*;
use bracket_parse::Bracket;
use sc_error::ScErr;
use std::collections::HashMap;


#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
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

    pub fn from_lz(lz:&Lz)->Result<CardKey,ScErr>{
        let kind = match lz.get_t::<CardType>("tp"){
            Ok(k)=>k,
            Err(LzErr::NoParse(_))=>return Err(ScErr::NoKind),
            _=>return Err(ScErr::NotFound),
        };
        Ok(CardKey{
            name:lz.name.clone(),
            kind:kind,
        });
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




#[derive(Clone,Copy,Debug,PartialEq,Serialize,Deserialize)]//,EnumFromStr)]
pub enum CardType{
    Goal,
    Role,
    Trait,
    Skill,
    Event,
    Scenario,
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
            r=>Err(ScErr::NoKind(r.to_string())),
        }
    }
}


pub fn load_data<F:AsRef<Path>>(fname:F)->Result<HashMap<CardKey,CardData>,ScErr>{
    let lst = LzList::load(fname)?;
    data_from_lzlist(&lst)
}

pub fn data_from_lzlist(dt:&LzList)->Result<HashMap<CardKey,CardData>,ScErr>{
    let mut res = HashMap::new();
    for d in dt {
        res.insert(CardKey::from_lz(d)?,CardData::from_lz(d)?);
    }
    Ok(res)
}


#[derive(Clone,Debug,PartialEq)]
pub struct CardData{
    pub text:String,
    pub cost:u8,
    pub count:u8,
}

impl CardData{
    pub fn from_lz(lz:&Lz)->Result<CardData,ScErr>{
        Ok(CardData{
            count:lz.get_t_def("ext0",1),
            text:lz.get_s_def("tx",""),
            cost:lz.get_t_def("Cost",4),
        })
    }
}

