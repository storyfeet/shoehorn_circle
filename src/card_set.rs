use std::collections::{hash_map,HashMap};
use std::path::Path;
use lazyf::{SGetter,Lz,LzList};
use sc_error::ScErr;
use card::CardKey;
use serde::ser::{SerializeStruct,SerializeMap,Serialize,Serializer};

#[derive(Clone,Debug,PartialEq,Deserialize)]
pub struct CardSet  (HashMap<CardKey,CardData>);


#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct CardData{
    pub text:String,
    pub cost:u8,
    pub count:u8,
    pub bskill:Option<String>,
    pub btrait:Option<String>,
}

impl CardData{
    pub fn from_lz(lz:&Lz)->Result<CardData,ScErr>{
        Ok(CardData{
            count:lz.get_t_def("ext0",1),
            text:lz.get_s_def("tx",""),
            cost:lz.get_t_def("Cost",4),
            bskill:lz.get_s("Skill"),
            btrait:lz.get_s("Trait"),
        })
    }
}



impl CardSet {
    pub fn load<F:AsRef<Path>>(fname:F)->Result<Self,ScErr>{
        let lst = LzList::load(fname)?;
        CardSet::from_lzlist(&lst)
    }

    pub fn from_lzlist(dt:&LzList)->Result<Self,ScErr>{
        let mut res = HashMap::new();
        for d in dt {
            res.insert(CardKey::from_lz(d)?,CardData::from_lz(d)?);
        }
        Ok(CardSet(res))
    }

    pub fn iter(&self)->hash_map::Iter<CardKey,CardData>{
        (&self.0).into_iter()
    }

    pub fn insert(&mut self,ck:CardKey,cd:CardData){
        self.0.insert(ck,cd);
    }
    pub fn get(&self,ck:&CardKey)->Option<&CardData>{
        self.0.get(ck)
    }
}

impl Serialize for CardSet{
    fn serialize<S>(&self,ser:S)->Result<S::Ok,S::Error>
        where S:Serializer
    {
        let mut s = ser.serialize_map(Some(self.0.len()))?;
        for (k,v) in &self.0 {
            s.serialize_entry(&k.to_string(),v)?;
        }
        s.end()
    }
}




