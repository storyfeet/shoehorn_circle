use card::{Card,CardType};
use card_deck::Deck;
use lazyf::{LzList};
use std::path::Path;

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
                CardType::Goal=>res.goals.push_bottom(c),
                CardType::Role=>res.roles.push_bottom(c),
                CardType::Skill=>res.skills.push_bottom(c),
                CardType::Trait=>res.traits.push_bottom(c),
                CardType::Event=>res.traits.push_bottom(c),
                CardType::Scenario=>res.scenarios.push_bottom(c),
            }
        }
        Ok(res)
    }

    pub fn vec_decks<'a>(&'a mut self)->Vec<&'a mut Deck<Card>>{
        vec![&mut self.goals,&mut self.roles,&mut self.traits,&mut self.skills,&mut self.events, &mut self.scenarios]
    }

    pub fn shuffle_decks(&mut self){
        for d in self.vec_decks(){
            d.shuffle_draw_pile();
        }
    }
}

#[derive(Debug,Clone)]
pub struct GrowthRow{
    per_row:usize,  
    pub skills:Vec<Card>,
    pub traits:Vec<Card>,
    pub goals:Vec<Card>,
}

impl GrowthRow{
    pub fn new(per_row:usize,sp:&mut Supply)->GrowthRow{
        let mut res = GrowthRow{
            per_row:per_row,
            skills:Vec::new(),
            traits:Vec::new(),
            goals:Vec::new(),
        };
        res.topup(sp);
        res
    }

    pub fn topup(&mut self,sp:&mut Supply){
        let d_up = self.per_row - self.skills.len();
        if d_up > 0 {
            self.skills.extend(&mut sp.skills.draw(d_up));
        }
        let d_up = self.per_row - self.traits.len();
        if d_up > 0 {
            self.traits.extend(&mut sp.skills.draw(d_up));
        }
        let d_up = self.per_row - self.goals.len();
        if d_up > 0 {
            self.goals.extend(&mut sp.goals.draw(d_up));
        }
    }
}

#[cfg(test)]
mod tests{
    use super::{Card,Supply,GrowthRow};
    
    #[test]
    fn loader(){
        println!("TESTING LOADER");
        let mut supply = Supply::load("card_data/cards.lz").unwrap();
        supply.shuffle_decks();
        for c in &supply.goals {
            println!("{}:{}",c.name,c.text);
        }
        //TODO work out something to actually test
//        assert!(false);
//
        let grow = GrowthRow::new(3,&mut supply);
    
        assert_eq!(grow.skills.len(),3);
    }
}

