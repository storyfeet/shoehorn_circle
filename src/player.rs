use card::{Card,CardType};
use supply::{Supply};
use action::{Action};


#[derive(Debug,Clone)]
pub struct Player{
    pub name:String,
    pub cards:Vec<Card>,
    tokens:u8,
    dice:u8,
}

impl Player { 
    pub fn new(name:&str,s:&mut Supply)->Player{
        let mut cards:Vec<Card> = Vec::new(); 
        cards.extend(&mut s.roles.draw(2));
        cards.extend(&mut s.goals.draw(3));
        cards.extend(&mut s.traits.draw(4));
        cards.extend(&mut s.skills.draw(4));


        Player{
            name:name.to_string(),
            cards:cards,
            tokens:0,
            dice:8,
        }
    }
		
    pub fn empty(name:&str)->Player{
        Player{
            name:name.to_string(),
            cards:Vec::new(),
            tokens:0,
            dice:8,
        }
    }

    pub fn as_actions(&self)->Vec<Action>{
        let mut res = Vec::new();
        res.push(Action::AddPlayer(self.name.clone()));
        for c in &self.cards {
            res.push(Action::PlayerDraw(self.name.clone(),c.into()));
        }
        res
    }

    pub fn role(&self)->&str{
        self.cards.iter().find(|c|c.kind == CardType::Role).map(|c|&c.name as &str).unwrap_or("NO-ROLE")
    }

}


#[cfg(test)]
mod tests {
    use card::{CardType};
    use player::Player;
    use supply::Supply;

    #[test]
    fn test_loadfilter(){
        let mut sp = Supply::load("card_data/cards.lz").unwrap();
        let p = Player::new("matt",&mut sp); 
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

    }
}

