// ok, lets start with parsing something
use std::str::Chars;
use SchemeObj::Val;
use SchemeObj::Lst;

enum SchemeObj {
    Lst(Vec<SchemeObj>),
    //Lst(int),
    Val(int)
}

fn parse_list(mut src : Chars)->SchemeObj{
    let mut list = Vec::new();
    loop{
        match src.next(){
            Some('(') => {
                list.push(parse_list(src));
            },
            Some(')') => {
                return Lst(list);
            },
            Some(c) => {
                list.push(parse_val(c,src));
            },
            None => {
                println!("unterminated list");
                return Lst(list);
            }
        }
    }
}

fn parse_val(c: char, src : Chars) -> SchemeObj {
    Val(0)
}

fn parse(src : &str) -> SchemeObj{
    let mut v = src.chars();
    let mut list = Vec::new();
    loop {
        
        match v.next() {
            Some( '(' ) => {
                list.push(parse_list(v));
            },
            Some(c) => {
                list.push(parse_val(c,v));
            },
            None => {
                return Lst(list)
            }
        }
        }

            
}

fn main(){
    parse("(hello world list)");
    println!("hello foobar!");
}
