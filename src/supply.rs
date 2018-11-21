use crate::card::{CardType,CardKey};
use crate::card_set::CardSet;
use crate::sc_error::ScErr;
use crate::action::{Action};

use card_deck::Deck;
use std::sync::Arc;


#[derive(PartialEq,Debug)]
pub struct Supply{
    pub goals:Deck<CardKey>,
    pub roles:Deck<CardKey>,
    pub traits:Deck<CardKey>, 
    pub skills:Deck<CardKey>,
    pub events:Deck<CardKey>,
    pub scenarios:Deck<CardKey>,
    pub growth:Vec<CardKey>,
    pub c_set:Arc<CardSet>, 
}


impl Supply {
    //creates an empty supply
    fn empty(cs:Arc<CardSet>)->Self{
        Supply{
            goals:Deck::build().done(), 
            roles:Deck::build().done(), 
            events:Deck::build().done(), 
            scenarios:Deck::build().done(), 
            traits:Deck::build().done(), 
            skills:Deck::build().done(), 
            growth:Vec::new(),
            c_set:cs,
        }
    }


    pub fn from_map(mp:Arc<CardSet>)->Supply{
        let mut res = Self::empty(mp.clone()); 
        for (k,_) in mp.iter() {
            res.deck_by_type(k.kind).push_bottom(k.clone());
        }
        res.shuffle_decks();
        res
    }


    pub fn deck_by_type<'a>(&'a mut self,kind:CardType)->&'a mut Deck<CardKey>{
        match kind {
            CardType::Goal=>&mut self.goals,
            CardType::Role=>&mut self.roles,
            CardType::Skill=>&mut self.skills,
            CardType::Trait=>&mut self.traits,
            CardType::Event=>&mut self.events,
            CardType::Scenario=>&mut self.scenarios,
        }
    }

    pub fn vec_decks<'a>(&'a mut self)->Vec<&'a mut Deck<CardKey>>{
        vec![&mut self.goals,&mut self.roles,&mut self.traits,&mut self.skills,&mut self.events, &mut self.scenarios]
    }

    pub fn shuffle_decks(&mut self){
        for d in self.vec_decks(){
            d.shuffle_draw_pile();
        }
    }

    pub fn discard(&mut self,c:CardKey){
        self.deck_by_type(c.kind).put_discard(c);
    }

    pub fn setup_growth(&mut self, per_row:usize)->Vec<Action>{

        for kind in [CardType::Skill,CardType::Trait,CardType::Goal].into_iter(){
            let dr:Vec<CardKey> = self.deck_by_type(*kind).draw(per_row).collect();
            self.growth.extend(dr);
        }

        let mut res = Vec::new(); 
        for c in &self.growth{
            res.push(Action::FillGrowth(c.clone())); 
        }
        res
    }

    pub fn redo_fill_growth(&mut self,ck:&CardKey)->Result<Action,ScErr>{
        let c = self.dig(ck)?;
        self.growth.push(c);
        Ok(Action::FillGrowth(ck.clone()))
    }

    pub fn fill_growth(&mut self,k:CardType)->Result<Action,ScErr>{
        let c = self.deck_by_type(k).draw_1().ok_or(ScErr::NoCards)?;
        self.growth.push(c.clone());
        Ok(Action::FillGrowth(c))
    }

    pub fn dig(&mut self,ck:&CardKey)->Result<CardKey,ScErr>{
        self.deck_by_type(ck.kind).dig_for(|c| c == ck).ok_or(ScErr::not_found(&ck.name))
    }

}

#[cfg(test)]
mod tests{
    //use super::{Supply};
    
    /*#[test] //TODO re-apply, once load works properly
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
    }*/
}

