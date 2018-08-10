use card::CardKey;
use std::str::FromStr;
use sc_error::ScErr;
//use itertools::Itertools;
use bracket_parse::Bracket;




#[derive(Debug,PartialEq,Clone)]
pub enum Action{
    AddPlayer(String),
    PlayerDraw(String,CardKey),
    Pl(PlAction),
    FillGrowth(CardKey),
    BuyGrowth(usize,CardKey,CardKey),//playernum, bought, topup
    Roll(String,Vec<u32>),//winner , Rolls
    WhoDunnitIs(usize,String), //dunnit playernum , What done
}


#[derive(Debug,PartialEq,Clone)]
pub struct PlAction{
    pub player_name:String,
    pub act:PlActionType,
}


#[derive(Debug,PartialEq,Clone)]
pub enum PlActionType{
    Chat(String),
    Do(String),
    Say(String),
    Bid(u8),//ndice
    WhoDunnit(String),//Text for what they done
    Reward(String,CardKey),//Player Card
    BuyGrowth(CardKey),
}

impl PlAction{
    pub fn new(nm:&str,a:PlActionType)->Self{
        PlAction{
            player_name:nm.to_string(),
            act:a,
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
        let h2 = brack.tail_h(1);
        let h3= brack.tail_h(2);

        match &h2.match_str().to_lowercase() as &str{
            "chat"=>
                Ok(PlAction::new(&username, Chat(h3.string_val()))), 
            "say"=>
                Ok(PlAction::new(&username, Say(h3.string_val()))), 
            "do"=>
                Ok(PlAction::new(&username, Do(h3.string_val()))), 
            "whodunnit"=>
                Ok(PlAction::new(&username, WhoDunnit(h3.string_val()))),
            "bid"=>
                Ok(PlAction::new(&username,Bid(h3.string_val().parse()?))),
            offlist=>Err(ScErr::NoParse(format!("Off List {}",offlist))),
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn action_create(){
        use self::PlActionType::*;
        assert_eq!(PlAction::from_str("Matt Chat \"hello everybody\"").unwrap(),PlAction::new("Matt",Chat("hello everybody".to_string())));
        
        assert_eq!(PlAction::from_str("Matt Bid 4").unwrap(),PlAction::new("Matt",Bid(4)));
    }
}


