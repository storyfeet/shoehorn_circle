use card_deck::Deck;
use lazyf::{LzList,SGetter,Lz};
use std::path::Path;
use std::str::FromStr;


pub struct Supply{
    pub goals:Deck<Card>,
    pub roles:Deck<Card>,
    pub events:Deck<Card>,
    pub scenarios:Deck<Card>,
    pub traits:Deck<Card>, 
    pub skills:Deck<Card>,

}

impl Supply {
    //creates an empty supply
    pub fn new()->Self{
        Supply{
            goals:Deck::new(), 
            roles:Deck::new(), 
            events:Deck::new(), 
            scenarios:Deck::new(), 
            traits:Deck::new(), 
            skills:Deck::new(), 
        }
    }
    pub fn load<P:AsRef<Path>>(fname:P)->Result<Supply,String>{
        let lzl = LzList::load(fname)?;
        let mut res = Self::new();
        for lz in lzl.iter() {
            let c = match Card::from_lz(lz){
                Ok(cv)=>cv,
                _=>continue,
            };
            
            match c.kind {
                Goal=>res.goals.push(c),
                Role=>res.roles.push(c),
                Skill=>res.skills.push(c),
                Trait=>res.traits.push(c),
                Scenario=>res.scenarios.push(c),
                Roles=>res.roles.push(c),
            }
        }
        Ok(res)
    }
}

pub enum CardType{
    Goal,
    Trait,
    Skill,
    Event,
    Scenario,
}

impl FromStr for CardType{
    type Err = String;
    fn from_str(s:&str)->Result<Self,Self::Err>{
        match s.to_lower(){
            "goal"=>Ok(Goal),
            "trait"=>(Trait),
            r=>Err(Format("Not a Card Type : {}",r)),
        }
    }
}

pub struct Card{
    name:String,
    text:String,
    kind:CardType,
    cost:u8,
    tokens:u8,
}

impl Card{
    pub fn from_lz(lz:&Lz)->Result<Card,String>{
        let kind = lz.get_t::<CardType>("tp")?;
        Ok(Card{
            name:lz.name,
            text:lz.get_s_def("tx",""),
            kind:kind,
            cost:lz.get_t_def("Cost",0),
            tokens:0,
        })
    }
}


#[cfg(test)]
mod tests{
    use super::Card;
    
    #[test]
    fn loader(){
        let supply = Supply::load("card_data/cards.lz");
        for c in &supply.goals {
            println!("{}:{}",c.name,c.text);
        }
    }

}

