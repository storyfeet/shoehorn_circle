use card::CardKey;
use std::str::FromStr;
use sc_error::ScErr;
//use itertools::Itertools;
use bracket_parse::Bracket;


pub enum Action{
    Pl(PlAction),
    FillSupply(CardKey),
    Roll(String),//winner
    WhoDunnitIs(String), 
}


#[derive(Debug,PartialEq)]
pub struct PlAction{
    player_name:String,
    does:PlActionType,
}


#[derive(Debug,PartialEq)]
pub enum PlActionType{
    Chat(String),
    Do(String),
    Say(String),
    Bid(u8),//ndice
    WhoDunnit(String),//Text for what they done
    Reward(String,CardKey),//Player Card
}

impl PlAction{
    pub fn new(nm:&str,a:PlActionType)->Self{
        PlAction{
            player_name:nm.to_string(),
            does:a,
        }
    }
}

    


impl FromStr for PlAction{
    type Err = ScErr;
    fn from_str(s:&str)->Result<Self,Self::Err>{
        use self::PlActionType::*;
        let brack:Bracket = s.parse()?;

        let username = match brack.head() {
            Bracket::Leaf(ref s)=>s.to_string(),
            _=>return Err(ScErr::NoParse("No name supplied".to_string())),
        };
        let t2= brack.tail();
        let t3= t2.tail();

        match &t2.head().match_str().to_lowercase() as &str{
            "chat"=>Ok(PlAction::new(&username, Chat(t3.head().match_str().to_string()))), 
            "say"=>Ok(PlAction::new(&username, Say(t3.head().match_str().to_string()))), 
            "do"=>Ok(PlAction::new(&username, Do(t3.head().match_str().to_string()))), 
            "whodunnit"=>
                Ok(PlAction::new(&username, WhoDunnit(t3.head()
                                     .match_str().to_string()))), 
            "bid"=>Ok(PlAction::new(&username,Bid(t3.head().match_str().parse()?))),
            offlist=>Err(ScErr::NoParse(format!("Off List {}",offlist))),
        }
    }
}


#[cfg(test)]
mod Tests{
    use super::*;
    #[test]
    fn action_create(){
        use self::PlActionType::*;
        assert_eq!(PlAction::from_str("Matt Chat \"hello everybody\"").unwrap(),PlAction::new("Matt",Chat("hello everybody".to_string())));
        
        assert_eq!(PlAction::from_str("Matt Bid 4").unwrap(),PlAction::new("Matt",Bid(4)));
    }
}


