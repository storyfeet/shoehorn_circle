//! This should not be here, I'll work out where to put it later, I just want to be able to add
//! an Insert on Iterators in General, I might even see if I can get it in stdlib at some point
//! It exists to insert a "space in a string iterator"
//!

use std::iter::Iterator;

enum Mode {
    Item,
    Join,
}

pub struct Joiner<IT,T>
    where IT:Iterator<Item=T>
{ 
    iter : IT,
    next : Option<T>,
    joiner : T,
    mode : Mode,
}

impl<IT,T> Joiner<IT,T>
    where IT : Iterator<Item=T>,
          T:Clone,
{
    pub fn join(mut iter:IT,t:T)->Joiner<IT,T>
    {
        let nx = iter.next();
        Joiner::<IT,T>{
            iter:iter,
            next:nx,
            joiner:t,
            mode: Mode::Item,
        }
    }
}

impl<IT,T> Iterator for Joiner<IT,T>
    where IT:Iterator<Item = T>,
          T:Clone,
{
    type Item = T;
    fn next(&mut self)->Option<Self::Item>{
        match self.mode {
            Mode::Item=>{
                self.mode = Mode::Join;
                let res = self.next;
                self.next = self.iter.next();
                res
            }
            Mode::Join=>{
                self.mode = Mode::Item;
                match self.next {
                    Some(ref n)=>Some(self.joiner.clone()),
                    None=>None,
                }
            }
        }
    }
}


#[cfg(test)]
mod Tests{
    use super::*;
    #[test]
    fn test_joiner(){
        let s = "hello people of the world".split(" ");
        let res:String = Joiner::join(s,",").collect();

        assert_eq!(res,"hello,people,of,the,world".to_string());
        
    }
}
