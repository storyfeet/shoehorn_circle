use std::fmt::{Display,Formatter};
use std::fmt;



#[derive(Debug)]
pub enum ScErr{
    NoLoad(String),
    NoParse(String),
    None_Err(),
    Other_Err(String),
}

impl From<String> for ScErr {
    fn from(s:String)->ScErr{
        ScErr::Other_Err(s)
    }
}


/*impl<S:AsRef<String>> From<S> for ScErr {
    fn from(s:S)->ScErr{
        let s:&str = s.as_ref();
        ScErr::Other_Err(s.to_string())
    }
}*/

impl Display for ScErr{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,"{:?}",self)
    }
}
