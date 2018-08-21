use std::collections::HashMap;
use std::path::Path;
use lazyf::{SGetter,Lz,LzList};
use sc_error::ScErr;
use card::CardKey;

pub type CardSet = HashMap<CardKey,CardData>;



#[derive(Clone,Debug,PartialEq)]
pub struct CardData{
    pub text:String,
    pub cost:u8,
    pub count:u8,
}

impl CardData{
    pub fn from_lz(lz:&Lz)->Result<CardData,ScErr>{
        Ok(CardData{
            count:lz.get_t_def("ext0",1),
            text:lz.get_s_def("tx",""),
            cost:lz.get_t_def("Cost",4),
        })
    }
}


pub fn load<F:AsRef<Path>>(fname:F)->Result<CardSet,ScErr>{
    let lst = LzList::load(fname)?;
    from_lzlist(&lst)
}

pub fn from_lzlist(dt:&LzList)->Result<CardSet,ScErr>{
    let mut res = HashMap::new();
    for d in dt {
        res.insert(CardKey::from_lz(d)?,CardData::from_lz(d)?);
    }
    Ok(res)
}

