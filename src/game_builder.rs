use ::{Game,Supply,Player,Action,ScErr};

use std::path::Path;



pub struct GameBuilder{
    nplayers:usize,
    g_row_size:usize,
    player_names:Option<Vec<String>>,
    supply:Option<Supply>,
    err:Option<ScErr>,
}

impl GameBuilder{
    pub fn new()->GameBuilder{
        GameBuilder{
            g_row_size:3,
            nplayers:4,
            player_names:None,
            supply:None,
            err:None,
        }
    }


    pub fn supply_file<P:AsRef<Path>>(mut self,fname:P)->Self{
        match Supply::load(fname){
            Ok(s)=>self.supply = Some(s),
            Err(e)=>self.err= Some(e),
        }
        self
    }

    pub fn supply(mut self,sp:Supply)->Self{
        self.supply=Some(sp);
        self
    }
    
    pub fn n_players(mut self, n:usize)->Self{
        self.nplayers = n;
        self
    }

    pub fn player_names<IT>(mut self,names:IT)->Self
        where IT:IntoIterator<Item=String>{
        self.player_names = Some(names.into_iter().collect());
        self
    }

    pub fn done(self)->Result<Game,ScErr>{
        if let Some(e)= self.err {
            return Err(e);
        }
        let pnames:Vec<String> = match self.player_names{
            Some(n)=>n,
            None=> (0..self.nplayers).map(|i|format!("P{}",i)).collect(),
        };

        let mut supply = match self.supply{
            Some(sp)=>sp,
            None=>Supply::load("card_data/cards.lz")?,
        };
        supply.shuffle_decks();

        let mut actions = supply.setup_growth(self.g_row_size);

        let mut players = Vec::new();
        for (i,nm) in pnames.iter().enumerate() {
            let p = Player::new(&nm,&mut supply);
            actions.extend(p.as_actions(i));
            players.push(p);
        }


        let mut res = Game{
            players:players,
            actions:actions,
            supply:supply,
        };

        Ok(res)
    }

    pub fn done_history(self,v:Vec<Action>)->Result<Game,ScErr>{
        if let Some(e)= self.err {
            return Err(e);
        }

        let mut supply = match self.supply{
            Some(sp)=>sp,
            None=>Supply::load("card_data/cards.lz")?,
        };
        supply.shuffle_decks();

        let mut res = Game{
            players:Vec::new(),
            actions:Vec::new(),
            supply:supply,
        };
        res.run_actions(v);
        Ok(res)
    }

}
