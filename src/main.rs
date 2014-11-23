use std::str::Chars;
use std::tuple::Tuple2;

trait Parser<'a, T:Clone> {
    fn m(&self, &mut Chars) -> (bool,T);
}

struct Match<'a, T:Clone> {
    c : char,
    pass: T,
    fail: T
        
}

impl<'a, T:Clone> Parser<'a, T> for Match<'a, T> {
    fn m(&self, src:&mut Chars) -> (bool,T){

        match src.next(){
            Some(v) => if v == self.c {
                (true,self.pass.clone())
            } else {
                (false,self.fail.clone())
            },
            None => (false,self.fail.clone())
        }
    }
}

struct Or<'a,T:Clone> {
    f: &'a Parser<'a,T> + 'a,
    s: &'a Parser<'a,T> + 'a
}

impl<'a,T:Clone> Parser<'a,T> for Or<'a,T> {
    fn m(&self, src:&mut Chars) -> (bool,T){

        let save = src.clone();
        let (x,t) = self.f.m(src);
        if x {
            (x,t)
        } else {
            *src = save;
            self.s.m(src)
        }
    }
}

struct And<'a,T:Clone> {
    f: &'a Parser<'a,T> + 'a,
    s: &'a Parser<'a,T> + 'a
}

impl<'a,T:Clone> Parser<'a,T> for And<'a,T> {
    fn m(&self, src:&mut Chars) -> (bool,T){
        let save = src.clone();
        let (x,t) = self.f.m(src);
        if x {
            let (x2,t2) = self.s.m(src);
            if x2 {
                (x2,t2)
            } else {
                *src = save;
                (x2,t2)
            }
        } else {
            *src = save;
            (x,t)
        }
    }
}


fn foo<'a,T:Clone>(p:&Parser<'a,T>){
    println!("ac {}", (p.m(&mut ("ac".chars()))).val0());
    println!("bc {}", (p.m(&mut ("bc".chars()))).val0());
}

fn main(){
    let o = And{
        f:&Or{f:&Match{c:'a',pass:true,fail:false},
             s:&Match{c:'b',pass:true,fail:false}},
        s:&Match{c:'c',pass:true,fail:false}
    };

    foo(&o);
}
