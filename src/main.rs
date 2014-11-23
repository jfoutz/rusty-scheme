#![feature(overloaded_calls)]


use std::str::Chars;

trait Parser<'a> : Fn<(&'a mut Chars<'a>,),bool> {}

struct Match{
    n: char,
}

impl<'a> Fn<(&'a mut Chars<'a>,), bool> for Match{
    extern "rust-call" fn call(&self, args: (& mut Chars<'a>,)) -> bool {
        let (v,) = args;
        match v.next(){
            Some(c) => self.n == c,
            None => false
        }
    }
}
impl<'a> Parser<'a> for Match {}


fn mk_match(c:char) -> Match{
    Match{n:c}
}

struct Or<'a,T: Parser<'a> >{
    f:&'a T,
    s:&'a T
}

impl<'a,T:Parser<'a>> Fn<(&'a mut Chars<'a>,), bool> for Or<'a,T>{
    extern "rust-call" fn call(&self, args: (& mut Chars<'a>,)) -> bool {
        let (v,) = args;
        let try = v.clone();
        if((*self.f)(try)){
            true
        } else {
            (*self.s)(v)
        }
    }
}
impl<'a> Parser<'a> for Or<'a,Parser<'a> +'a> {}

fn main(){
    let m = mk_match('a');
    let mut a = "a".chars();
    let mut b = "b".chars();
    println!("hello {}!", m(&mut a));
    println!("hello {}!", m(&mut b));
}
