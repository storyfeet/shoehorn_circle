extern crate shoehorn_circle as shoehorn;
use shoehorn::Game;
use shoehorn::card::{CardType,CardKey};
use shoehorn::action::Action;
//use shoehorn::supply::Supply;
//use shoehorn::player::Player;

use std::io;


fn main(){
    let mut gm = Game::build().supply_file("card_data/cards.lz").player_names(vec!["Matt".to_string(),"Toby".to_string()]).done().expect("Game not loaded");


    let stdin = io::stdin();

    for i in 0..gm.players.len(){ 

        let mut actions = Vec::new();
        {//borrow player

            let p = &gm.players[i];

            println!("{}:" ,p.name);
            let mut s = String::new();
            stdin.read_line(&mut s).expect("Could not read Line");
            for c in &p.cards {
                println!("  {}:{:?}:{}",c.name,c.kind,c.text);
            }

            let ctypes = vec![CardType::Goal,CardType::Role,CardType::Trait,CardType::Skill];

            for t in ctypes {
                println!("TRAIT : {:?}",t);
                let mut keys:Vec<CardKey> = Vec::new();
                for (i,a) in p.cards.iter()
                            .filter(|c| c.kind == t).enumerate(){
                    println!("   {}:{}",i,a.name);
                    keys.push(a.into());
                }

                if keys.len() < 2 { continue}
                let mut ln = String::new();
                println!("Drop Which?>");
                stdin.read_line(&mut ln).expect("Could not read Line");
                let dropn = ln.trim().parse::<usize>().unwrap_or(0);

                if dropn >= keys.len() {continue} //consider later

                let key = &keys[dropn];

                println!("DROPPING {}",key.name);
                actions.push(Action::DropCard(p.p_num,key.clone()));
            }
            
        }// Borrow player

        gm.run_actions(actions).expect("drop actions failed running actions");

        println!("Keeping:");
        for c in &gm.players[i].cards {
            println!("  {:?}::{}:{}",c.kind,c.name,c.text);
        }

    }//for i in players.len
}
