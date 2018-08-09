use card::{Card,CardType};
use card_deck::Deck;
use lazyf::{LzList};
use std::path::Path;
use sc_error::ScErr;
use action::{Action};
use std;


pub struct Supply{
    pub goals:Deck<Card>,
    pub roles:Deck<Card>,
    pub traits:Deck<Card>, 
    pub skills:Deck<Card>,
    pub events:Deck<Card>,
    pub scenarios:Deck<Card>,
    pub growth:Vec<Card>,
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
            growth:Vec::new(),
        }
    }
    pub fn load<P:AsRef<Path>>(fname:P)->Result<Supply,ScErr>{
        let lzl = LzList::load(fname)?;
        let mut res = Self::new();
        for lz in lzl.iter() {
            let c = match Card::from_lz(lz){
                Ok(cv)=>cv,
                _=>continue,
            };
            
            res.deck_by_type(c.kind).push_bottom(c);
        }
        Ok(res)
    }

    pub fn deck_by_type<'a>(&'a mut self,kind:CardType)->&'a mut Deck<Card>{
        match kind {
            CardType::Goal=>&mut self.goals,
            CardType::Role=>&mut self.roles,
            CardType::Skill=>&mut self.skills,
            CardType::Trait=>&mut self.traits,
            CardType::Event=>&mut self.events,
            CardType::Scenario=>&mut self.scenarios,
        }
    }

    pub fn vec_decks<'a>(&'a mut self)->Vec<&'a mut Deck<Card>>{
        vec![&mut self.goals,&mut self.roles,&mut self.traits,&mut self.skills,&mut self.events, &mut self.scenarios]
    }

    pub fn shuffle_decks(&mut self){
        for d in self.vec_decks(){
            d.shuffle_draw_pile();
        }
    }

    pub fn discard(&mut self,c:Card){
        self.deck_by_type(c.kind).put_discard(c);
    }

    pub fn setup_growth(&mut self, per_row:usize)->Vec<Action>{

        for kind in [CardType::Skill,CardType::Trait,CardType::Goal].into_iter(){
            let mut dr:Option<Vec<Card>> = None;
            {
                dr = Some(self.deck_by_type(*kind).draw(per_row).collect());
            }
            self.growth.extend(dr.unwrap());
        }

        let mut res = Vec::new(); 
        for c in &self.growth{
            res.push(Action::FillGrowth(c.into())); 
        }
        res
    }

}

#[cfg(test)]
mod tests{
    use super::{Supply};
    
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
        supply.setup_growth(3);
    
        assert_eq!(supply.growth.len(),9);
    }
}

