mod interpreter;
mod stacktypes;
use interpreter::*;
pub fn run(){
    let mut i: Interpreter;
    i = Interpreter::new();
    let program = "1 5 + 1 + 5 + 2 + 3 + .";
    let parse = program.split(" ");
    let mut length: usize = 0;
    for _ in parse.clone(){
        length+=1;
    }
    let mut iter: usize = 0;
    while iter < length{
        let tok = parse.clone().nth(iter).unwrap();
        let int: i32;
        let isint;
        isint = tok.parse::<i32>().is_ok();
        if isint{
            int = tok.parse::<i32>().unwrap();
            push!(i,int);
            iter+=1;
            continue;
        }
        if tok == "+"{
            add!(i);
        }
        if tok == "."{
            println!("{}",pop!(i));
        }
        iter+=1;
    }
}

fn main(){
    run();
}