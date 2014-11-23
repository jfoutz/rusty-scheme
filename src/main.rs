// ok, lets start with parsing something
use std::str::Chars;
use std::fmt;
use SchemeObj::Val;
use SchemeObj::Lst;

enum SchemeObj {
    Lst(Vec<SchemeObj>),
    Val(String)
}

impl fmt::Show for SchemeObj{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            &Val(ref v) => write!(f, "{}", v),
            &Lst(ref v) => {
                write!(f, "({}) ", v)
            }
        }
    }
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
                println!("matched {}", c);
                list.push(parse_val(c,&mut src));
            },
            None => {
                println!("unterminated list");
                return Lst(list);
            }
        }
    }
}

fn parse_val(c: char, src : &mut Chars) -> SchemeObj {
    let mut sym = String::new();
    sym.push(c);
    loop {
        match src.next(){
            Some(' ') => {
                return Val(sym);
            }
            Some('\n') => {
                return Val(sym);
            }
            Some(c) => {
                sym.push(c);
            }
            None => {
                return Val(sym);
            }
        }
    }
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
                list.push(parse_val(c,&mut v));
            },
            None => {
                return Lst(list)
            }
        }
        }

            
}

fn main(){
    let s = parse("(hello world list)");
    println!("hello {}!", s);
}
