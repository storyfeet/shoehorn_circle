use std::fmt::{Display,Formatter};
use std::num::ParseIntError;
use std::fmt;



#[derive(Debug)]
pub enum ScErr{
    NoLoad(String),
    NoParse(String),
    NotFound(String),
    OtherErr(String),
    NotGm(String),
    NoDice,
    NoToken,
    NoCards,
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


impl ScErr{
    pub fn no_load(s:&str)->ScErr{
        ScErr::NoLoad(s.to_string())
    }
    pub fn not_found(s:&str)->ScErr{
        ScErr::NotFound(s.to_string())
    }
    pub fn not_gm(s:&str)->ScErr{
        ScErr::NotGm(s.to_string())
    }
}


impl Display for ScErr{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"{:?}",self)
    }
}
