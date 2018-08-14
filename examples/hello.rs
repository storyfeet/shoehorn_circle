extern crate shoehorn_circle as shoehorn;
use shoehorn::{Game,ScErr};
use shoehorn::card::{CardType,CardKey};
use shoehorn::action::Action;
//use shoehorn::supply::Supply;
//use shoehorn::player::Player;

use std::io;



fn setup()->Result<Game,ScErr>{

    let mut gm = Game::build().supply_file("card_data/cards.lz").player_names(vec!["Matt".to_string(),"Toby".to_string()]).done()?;


    let stdin = io::stdin();

    for i in 0..gm.get_players().len(){ 

        let mut actions = Vec::new();
        {//borrow player

            let p = &gm.get_players()[i];

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
                let mut dropn:Option<usize> = None;

                while dropn == None{
                    for (i,a) in p.cards.iter()
                                .filter(|c| c.kind == t).enumerate(){
                        println!("   {}:{}",i,a.name);
                        keys.push(a.into());
                    }

                    if keys.len() < 2 { continue}
                    let mut ln = String::new();
                    println!("Drop Which?>");
                    stdin.read_line(&mut ln).expect("Could not read Line");
                    dropn = ln.trim().parse::<usize>().ok();
                    
                    if dropn >= Some(keys.len()) {
                        dropn = None; 
                    }
                }

                let key = &keys[dropn.unwrap()];

                println!("DROPPING {}",key.name);
                actions.push(Action::DropCard(p.p_num,key.clone()));
            }
            
        }// Borrow player

        gm.run_actions(actions).expect("drop actions failed running actions");

        println!("Keeping:");
        for c in &gm.get_players()[i].cards {
            println!("  {:?}::{}:{}",c.kind,c.name,c.text);
        }

    }//for i in players.len
    Ok(gm)
}


fn main(){
    let _gm = setup().unwrap();

}
