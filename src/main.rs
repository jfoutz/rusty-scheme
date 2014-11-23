use std::str::Chars;

trait Parser<'a> {
    fn m(&self, &mut Chars) -> bool;
}

struct Match<'a> {
    c : char
}

impl<'a> Parser<'a> for Match<'a> {
    fn m(&self, src:&mut Chars) -> bool{
        println!("match {} ", self.c);
        match src.next(){
            Some(v) => v == self.c,
            None => false
        }
    }
}

struct Or<'a> {
    f: &'a Parser<'a> + 'a,
    s: &'a Parser<'a> + 'a
}

impl<'a> Parser<'a> for Or<'a> {
    fn m(&self, src:&mut Chars) -> bool{
        println!("or ");
        let mut save = src.clone();
        if self.f.m(src){
            true
        } else {
            self.s.m(&mut save)
        }
    }
}

struct And<'a> {
    f: &'a Parser<'a> + 'a,
    s: &'a Parser<'a> + 'a
}

impl<'a> Parser<'a> for And<'a> {
    fn m(&self, src:&mut Chars) -> bool{
        println!("and ");
        if self.f.m(src){
            self.s.m(src)
        } else {
            false
        }
    }
}


fn foo(p:&Parser){
    println!("ac {}", p.m(&mut ("ac".chars())));
    println!("bc {}", p.m(&mut ("bc".chars())));
}

fn main(){
    let o = And{
        f:&Or{f:&Match{c:'a'},
             s:&Match{c:'b'}},
        s:&Match{c:'c'}
    };
    let c = & mut("abc".chars());
    o.m(c);

    o.m(&mut ("def".chars()));
    foo(&o);
}
