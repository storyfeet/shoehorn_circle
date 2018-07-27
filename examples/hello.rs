extern crate shoehorn_circle;
use shoehorn_circle::card::{Supply,Card,CardType};
use shoehorn_circle::player::Player;


fn main(){
    let mut sp = Supply::load("card_data/cards.lz").expect("Could not load cards");
    let mut players = Vec::new();
    for i in 0..4 {
        players.push(Player::new(&format!("p{}",i),&mut sp));
    }

    for p in players {
        println!("{}:" ,p.username);
        for c in p.cards {
            println!("  {}:{:?}:{}",c.name,c.kind,c.text);
        }
    }
    
}
