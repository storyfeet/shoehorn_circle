extern crate shoehorn_circle as shoehorn;
use shoehorn::Game;
use shoehorn::card::{Card,CardType};
use shoehorn::supply::Supply;
use shoehorn::player::Player;

use std::io;


fn main(){
    let mut gm = Game::build().supply_file("card_data/cards.lz").player_names(vec!["Matt".to_string(),"Toby".to_string()]).done().expect("Game not loaded");


    let mut stdin = io::stdin();

    for p in &mut gm.players {

        println!("{}:" ,p.username);
        let mut s = String::new();
        stdin.read_line(&mut s);
        for c in &p.cards {
            println!("  {}:{:?}:{}",c.name,c.kind,c.text);
        }


        let ctypes = vec![CardType::Goal,CardType::Role,CardType::Trait,CardType::Skill];

        for t in ctypes {
            println!("TRAIT : {:?}",t);
            let mut len = 0;
            for a in p.cards.iter().filter(|c| c.kind == t){
                println!("   {}:{}",len,a.name);
                len += 1;
            }
            if len < 2 { continue}
            let mut ln = String::new();
            println!("Drop Which?>");
            stdin.read_line(&mut ln).expect("Could not read Line");
            let dropn = ln.trim().parse::<usize>().unwrap_or(0)%len;
            
            let mut curr = 0;
            p.cards.retain(|ref c| {
                if c.kind != t {
                    return true;
                }
                if curr == dropn {
                    println!("DROPPING {}",c.name);
                    curr +=1;
                    return false;
                }
                curr +=1;
                true
            });
        }

        println!("Keeping:");
        for c in &p.cards {
            println!("  {:?}::{}:{}",c.kind,c.name,c.text);
        }
    }
}
