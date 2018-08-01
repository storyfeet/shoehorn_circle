use card::Card;
use std::str::FromStr;
use sc_error::ScErr;

#[derive(Debug,PartialEq)]
pub struct Action{
    player_name:String,
    does:ActionType,
}

#[derive(Debug,PartialEq)]
pub enum ActionType{
    Chat(String),
    Do(String),
    Say(String),
    FillSupply(Card),
    Bid(String,u8),
    Roll(String),//winner
    WhoDunnit(String),
    Reward(String,String),//Player Card
}

impl Action{
    pub fn new(nm:&str,a:ActionType)->Self{
        Action{
            player_name:nm.to_string(),
            does:a,
        }
    }
}

    


impl FromStr for Action{
    type Err = ScErr;
    fn from_str(s:&str)->Result<Self,Self::Err>{
        use self::ActionType::*;
        let mut ss = s.split(" ");
        let user = ss.next().expect("A Split string should always have one elem");
        match &ss.next()
                .ok_or(ScErr::NoParse("No Action name".to_string()))?
                .to_lowercase() as &str{
            "chat"=>Ok(Action::new(user,
                                   Chat(
                                       ss.map(|s| {
                                           let mut r =" ".to_string();
                                           r.push_str(s);
                                           r
                                       }).collect())
                                   )
                       ), 
            offlist=>Err(ScErr::NoParse(offlist.to_string())),

        }
    }
}


#[cfg(test)]
mod Tests{
    use super::*;
    #[test]
    fn test_create(){
        use self::ActionType::*;
        assert_eq!(Action::from_str("Matt Chat hello everybody").unwrap(),Action::new("Matt",Chat("hello everybody".to_string())));
    }
}


