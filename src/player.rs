use card::{CardType,CardKey};// TODO Use Card DATA for buy
use supply::{Supply};
use action::{Action};
use sc_error::ScErr;


#[derive(Debug,Clone,PartialEq)]
pub struct Player{
    pub name:String,
    pub p_num:usize,
    pub cards:Vec<CardKey>,
    pub tokens:Vec<CardKey>,
    pub dice:u8,
}


impl Player { 
    pub fn new(name:&str,pnum:usize,s:&mut Supply)->Player{
        let mut cards:Vec<CardKey> = Vec::new(); 
        cards.extend(&mut s.roles.draw(2));
        cards.extend(&mut s.goals.draw(3));
        cards.extend(&mut s.traits.draw(4));
        cards.extend(&mut s.skills.draw(4));

        Player{
            name:name.to_string(),
            p_num:pnum,
            cards:cards,
            tokens:Vec::new(),
            dice:8,
        }
    }
		
    pub fn empty(name:&str,pnum:usize)->Player{
        Player{
            name:name.to_string(),
            p_num:pnum,
            cards:Vec::new(),
            tokens:Vec::new(),
            dice:8,
        }
    }

    pub fn as_actions(&self)->Vec<Action>{
        let mut res = Vec::new();
        res.push(Action::AddPlayer(self.name.clone()));
        for c in &self.cards {
            res.push(Action::PlayerDraw(self.p_num,c.clone()));
        }
        res
    }

    pub fn role(&self)->&str{
        self.cards.iter().find(|c|c.kind == CardType::Role).map(|c|&c.name as &str).unwrap_or("NO-ROLE")
    }

    pub fn reward(&mut self, ckey:CardKey,ndice:u8)->Action{
        self.dice += ndice;
        self.tokens.push(ckey.clone());
        Action::Reward(self.p_num,ckey,ndice)
    }

    pub fn drop_card(&mut self,key:&CardKey,supply:&mut Supply)->Result<Action,ScErr>{
        //find
        let mut found:Option<usize> = None;
        for (i,c)in self.cards.iter().enumerate(){
            if *c == *key {
                found = Some(i); 
                break;
            }
        }
        match found {
            Some(n)=>{
                supply.discard(self.cards.remove(n));
                Ok(Action::DropCard(self.p_num,key.clone()))
            }
            _=>Err(ScErr::not_found(&key.name))
        }
        
    }

    pub fn buy_growth(&mut self,buy_key:&CardKey,pay_key:&CardKey,spp:&mut Supply)->Result<Action,ScErr>{

        let c_cost = spp.c_set.get(buy_key).ok_or(ScErr::NotFound)?.cost;
        let c_loc = spp.growth.iter()
                            .enumerate()
                            .find(|(_,c)|**c==*buy_key)
                            .map(|(i,_)|i)
                            .ok_or(ScErr::not_found(&buy_key.name))?;
        
        if c_cost > self.dice {
            return Err(ScErr::NoDice);
        }

        let tk_loc = self.tokens.iter()
                            .enumerate()
                            .find(|(_,k)|**k ==*pay_key)
                            .map(|(i,_)|i).ok_or(ScErr::NoToken)?;

        
        let bcard = spp.growth.remove(c_loc);

        self.cards.push(bcard.clone()); 
        let tk_key = self.tokens.remove(tk_loc);
        self.dice -= c_cost;
        Ok(Action::BuyGrowth(self.p_num,bcard,tk_key))

    }

}


#[cfg(test)]
mod tests {
    //use card::{CardType};
    //use player::Player;
    //use supply::Supply;

    /*#[test] TODO Fix program first
    fn test_loadfilter(){
        let mut sp = Supply::load("card_data/cards.lz").unwrap();
        let p = Player::new("matt",0,&mut sp); 
        assert_eq!(p.cards.len(),13,"{:?}",p);

        let mut tot = 0;
        for c in p.cards.iter().filter(|x| x.kind== CardType::Role) {
            tot += 1;
//            println!("{}:{}",c.name,c.text);
            for sc in &sp.roles{
                assert!(c.name != sc.name,"c.name = {}, sc.name= {}",c.name,sc.name);
            }
        }
        assert_eq!(tot,2,"Should have 2 cards for Role Check");

    }*/
}

