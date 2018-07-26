use card_deck::Deck;
use lazyf::{LzList,SGetter,Lz};
use std::path::Path;
use std;
use std::str::FromStr;


use self::CardType::*;


pub struct Supply{
    pub goals:Deck<Card>,
    pub roles:Deck<Card>,
    pub traits:Deck<Card>, 
    pub skills:Deck<Card>,
    pub events:Deck<Card>,
    pub scenarios:Deck<Card>,
}


impl Supply {
    //creates an empty supply
    pub fn new()->Self{
        Supply{
            goals:Deck::build().done(), 
            roles:Deck::build().done(), 
            events:Deck::build().done(), 
            scenarios:Deck::build().done(), 
            traits:Deck::build().done(), 
            skills:Deck::build().done(), 
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
                Goal=>res.goals.push_bottom(c),
                Role=>res.roles.push_bottom(c),
                Skill=>res.skills.push_bottom(c),
                Trait=>res.traits.push_bottom(c),
                Event=>res.traits.push_bottom(c),
                Scenario=>res.scenarios.push_bottom(c),
            }
        }
        Ok(res)
    }

    pub fn vec_decks<'a>(&'a mut self)->Vec<&'a mut Deck<Card>>{
        vec![&mut self.goals,&mut self.roles]
    }

    pub fn shuffle_decks(&mut self){
        for d in self.vec_decks(){
            d.shuffle_draw_pile();
        }
    }
}

#[derive(Clone,Copy)]//,EnumFromStr)]
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

pub struct Card{
    name:String,
    text:String,
    kind:CardType,
    cost:u8,
    tokens:u8,
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


#[cfg(test)]
mod tests{
    use super::{Card,Supply};
    
    #[test]
    fn loader(){
        println!("TESTING LOADER");
        let mut supply = Supply::load("card_data/cards.lz").unwrap();
        //supply.shuffle_decks();
        for c in &supply.goals {
            println!("{}:{}",c.name,c.text);
        }
        assert!(false);
        
    }

}

