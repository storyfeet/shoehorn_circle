use card::CardKey;
use std::str::FromStr;
use sc_error::ScErr;
//use itertools::Itertools;
use bracket_parse::Bracket;

pub type PlayerRef = usize;


#[derive(Debug,PartialEq,Clone)]
pub enum Action{
    Chat(PlayerRef,String),//Username, Says
    Do(PlayerRef,String),//Chat 
    Say(PlayerRef,String),
    Bid(PlayerRef,u8),
    Reward(PlayerRef,CardKey,u8),
    AddPlayer(String),
    PlayerDraw(PlayerRef,CardKey),
    FillGrowth(CardKey),
    BuyGrowth(PlayerRef,CardKey),//player, bought
    Roll(String,Vec<u32>),//winner , Rolls
    WhoDunnitIs(PlayerRef,String), //dunnit playernum , What done
    Fail(String),
}


#[derive(Debug,PartialEq,Clone)]
pub struct Request{
    pub player_name:String,
    pub act:RequestType,
}


#[derive(Debug,PartialEq,Clone)]
pub enum RequestType{
    Chat(String),
    Do(String),
    Say(String),
    Bid(u8),//ndice
    WhoDunnit(String),//Text for what they done
    Reward(String,CardKey,u8),//Player, Card, dice
    BuyGrowth(CardKey,CardKey),//Buy what, token from
}

impl Request{
    pub fn new(nm:&str,a:RequestType)->Self{
        Request{
            player_name:nm.to_string(),
            act:a,
        }
    }

}

    


impl FromStr for Request{
    type Err = ScErr;
    fn from_str(s:&str)->Result<Self,Self::Err>{
        use self::RequestType::*;
        let brack:Bracket = s.parse()?;

        let username = match brack.head() {
            Bracket::Leaf(ref s)=>s.to_string(),
            _=>return Err(ScErr::NoParse("No name supplied".to_string())),
        };
        let h2 = brack.tail_h(1);
        let h3= brack.tail_h(2);

        match &h2.match_str().to_lowercase() as &str{
            "chat"=>
                Ok(Request::new(&username, Chat(h3.string_val()))), 
            "say"=>
                Ok(Request::new(&username, Say(h3.string_val()))), 
            "do"=>
                Ok(Request::new(&username, Do(h3.string_val()))), 
            "whodunnit"=>
                Ok(Request::new(&username, WhoDunnit(h3.string_val()))),
            "bid"=>
                Ok(Request::new(&username,Bid(h3.string_val().parse()?))),
            offlist=>Err(ScErr::NoParse(format!("Off List {}",offlist))),
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn action_create(){
        use self::RequestType::*;
        assert_eq!(Request::from_str("Matt Chat \"hello everybody\"").unwrap(),Request::new("Matt",Chat("hello everybody".to_string())));
        
        assert_eq!(Request::from_str("Matt Bid 4").unwrap(),Request::new("Matt",Bid(4)));
    }
}


