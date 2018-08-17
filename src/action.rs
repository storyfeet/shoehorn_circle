use card::CardKey;
use std::str::FromStr;
use sc_error::ScErr;
//use itertools::Itertools;
use bracket_parse::Bracket;

pub type PlayerRef = usize;



#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub enum Action{
    Chat(PlayerRef,String),
    Do(PlayerRef,String),
    Say(PlayerRef,String), 
    Bid(PlayerRef,u8),//num dice
    Reward(PlayerRef,CardKey,u8),
    AddPlayer(String),
    PlayerDraw(PlayerRef,CardKey),
    FillGrowth(CardKey),
    BuyGrowth(PlayerRef,CardKey,CardKey),//player, bought, token from
    Roll(Vec<u32>),//Rolls
    WhoDunnitIs(PlayerRef,String), //dunnit playernum , What done
    DropCard(PlayerRef,CardKey),
}

pub fn roll_winner(v:&Vec<u32>)->usize{
    let mut winner:usize = 0;
    let mut top:u32 = 0;
    for (i,n) in v.iter().enumerate() {
        if *n > top {
            winner = i;
            top = *n;
        }
    }
    return winner;
}

#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub struct Request{
    pub player_name:String,
    pub act:RequestType,
}


#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
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

    pub fn from_bracket(brack:Bracket)->Result<Self,ScErr>{
        use self::RequestType::*;
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
            "reward"=>{
                let ck = CardKey::from_bracket(brack.tail_h(3))?;
                let ndice:u8 = brack.tail_h(4).string_val().parse()?;
                Ok(Request::new(&username,Reward(h3.string_val(),ck,ndice)))
            },
            "buy"=>{
                let ck = CardKey::from_bracket(h3)?;
                let cfrom = CardKey::from_bracket(brack.tail_h(3))?;
                Ok(Request::new(&username,BuyGrowth(ck,cfrom)))
            }
            
            offlist=>Err(ScErr::NoParse(format!("Off List {}",offlist))),
        }
    }

}

    


impl FromStr for Request{
    type Err = ScErr;
    fn from_str(s:&str)->Result<Self,Self::Err>{
        let brack:Bracket = s.parse()?;
        Self::from_bracket(brack)

    }
}


#[cfg(test)]
mod tests{
    use super::*;
    use serde_json;
    use card::{CardType,CardKey};
    #[test]
    fn action_create(){
        use self::RequestType::*;
        assert_eq!(Request::from_str("Matt Chat \"hello everybody\"").unwrap(),Request::new("Matt",Chat("hello everybody".to_string())));
        
        assert_eq!(Request::from_str("Matt Bid 4").unwrap(),Request::new("Matt",Bid(4)));

        assert_eq!(
            Request::from_str("Matt Reward Toby (Swordsman Skill) 3").unwrap(),
            Request::new("Matt",Reward("Toby".to_string(),CardKey::new("Swordsman",CardType::Skill),3)));

    }

    #[test]
    fn serdize(){
        let r = Request::from_str("Matt Chat \"hello everybody\"").unwrap();

        let rout = serde_json::to_string(&r).unwrap();

        let rjson = r#"{"player_name":"Matt","act":{"Chat":"hello everybody"}}"#;

        assert_eq!(rout,rjson);

        let r2:Request = serde_json::from_str(rjson).unwrap();

        assert_eq!(r,r2);

        
    }
}


