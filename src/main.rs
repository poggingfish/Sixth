mod interpreter;
mod stacktypes;
use std::io::Write;

use interpreter::*;
pub fn run(mut i: Interpreter, program: &str){
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
        else if tok == "-"{
            subtract!(i);
        }
        else if tok == "*"{
            times!(i);
        }
        else if tok == "/"{
            divide!(i);
        }
        else if tok == "."{
            println!("{}",pop!(i));
            let _ = std::io::stdout().flush();
        }
        iter+=1;
    }
}

fn main(){
    let i: Interpreter;
    i = Interpreter::new();
    run(i, "1 1 + 1 + 2 * .");
}