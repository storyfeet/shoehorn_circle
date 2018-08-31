use ::{Game,Supply,Player,Action,ScErr};
//use std::sync::Arc;



pub struct GameBuilder{
    nplayers:usize,
    g_row_size:usize,
    player_names:Option<Vec<String>>,
    supply:Supply,
}

impl GameBuilder{
    pub fn new(sp:Supply)->GameBuilder{
        GameBuilder{
            g_row_size:3,
            nplayers:4,
            player_names:None,
            supply:sp,
        }
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

    pub fn done(self)->Game{
        let pnames:Vec<String> = match self.player_names{
            Some(n)=>n,
            None=> (0..self.nplayers).map(|i|format!("P{}",i)).collect(),
        };

        let mut supply = self.supply;

        let mut actions = supply.setup_growth(self.g_row_size);

        let mut players = Vec::new();
        for (i,nm) in pnames.iter().enumerate() {
            let p = Player::new(&nm,i,&mut supply);
            actions.extend(p.as_actions());
            players.push(p);
        }

        Game{
            players:players,
            actions:actions,
            supply:supply,
        }
    }

    pub fn done_history(self,v:Vec<Action>)->Result<Game,ScErr>{
        let mut res = Game{
            players:Vec::new(),
            actions:Vec::new(),
            supply:self.supply,
        };
        res.run_actions(v)?;
        Ok(res)
    }

}
