use std::fmt::{Display,Formatter};
use std::num::ParseIntError;
use std::fmt;



#[derive(Debug)]
pub enum ScErr{
    NoLoad(String),
    NoParse(String),
    OtherErr(String),
}

impl From<String> for ScErr {
    fn from(s:String)->ScErr{
        ScErr::OtherErr(s)
    }
}

impl From<ParseIntError> for ScErr{
    fn from(e:ParseIntError)->ScErr{
        ScErr::NoParse(format!("{:?}",e))
    }
}


impl Display for ScErr{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"{:?}",self)
    }
}
