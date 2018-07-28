use card::Card;

pub struct BasicAction{
    name:String,
    text:String,
}

pub enum Action{
    Chat(BasicAction),
    Do(BasicAction),
    FillSupply(Card),
    Say(BasicAction),
    Bid(String,u8),
    Roll(String),//winner
    Reward(),
}
