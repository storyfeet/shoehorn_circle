extern crate shoehorn_circle as shoehorn;
use shoehorn::Game;
use shoehorn::card::{Card,CardType};
use shoehorn::supply::Supply;
use shoehorn::player::Player;

use std::io;


fn main(){
    let mut gm = Game::build().supply_file("card_data/cards.lz").player_names(vec!["Matt".to_string(),"Toby".to_string()]).done().expect("Game not loaded");


    for p in &mut gm.players {
        println!("{}:" ,p.username);
        for c in &p.cards {
            println!("  {}:{:?}:{}",c.name,c.kind,c.text);
        }
        let mut stdin = io::stdin();
        p.cards.retain(|ref c| {
            println!("  Drop Card : {} ?>",c.name);
            let mut s = String::new();
            stdin.read_line(&mut s).expect("Could not read Line");
            ! s.trim().parse::<bool>().unwrap_or(false)
        });

        println!("Keeping:");
        for c in &p.cards {
            println!("  {}:{:?}:{}",c.name,c.kind,c.text);
        }
    }
}
