use std::{collections::HashMap, io::Write};

use crate::stacktypes::{*};
#[macro_export]
macro_rules! push {
    ($i: tt,$f:tt) => {
        $i = Interpreter::push_int($i.clone(),$f)
    };
}
#[macro_export]
macro_rules! spush {
    ($i: tt,$f:tt) => {
        $i = Interpreter::push_str($i.clone(),$f)
    };
}
#[macro_export]
macro_rules! pop {
    ($i: tt) => {
        $i.stack.pop().unwrap()
    };
}
#[macro_export]
macro_rules! new_var {
    ($a: tt, $b: tt, $c: tt) => {
        $a = Interpreter::new_var($a, $b, $c);
    };
}
#[macro_export]
macro_rules! add {
    ($p: tt) => {
        let x = pop!($p);
        let y = pop!($p);
        if y.selected == 1{
            let z = y.inttype.unwrap() + x.inttype.unwrap();
            push!($p,z);
        }
        else{
            let z = y.strtype.unwrap() + &x.strtype.unwrap();
            spush!($p,z);
        }
    };
}
#[macro_export]
macro_rules! subtract {
    ($p: tt) => {
        let x = pop!($p);
        let y = pop!($p);
        let z = y.inttype.unwrap() - x.inttype.unwrap();
        push!($p,z);
    };
}
#[macro_export]
macro_rules! times {
    ($p: tt) => {
        let x = pop!($p);
        let y = pop!($p);
        let z = y.inttype.unwrap() * x.inttype.unwrap();
        push!($p,z);
    };
}
#[macro_export]
macro_rules! divide {
    ($p: tt) => {
        let x = pop!($p);
        let y = pop!($p);
        let z = y.inttype.unwrap() / x.inttype.unwrap();
        push!($p,z);
    };
}
#[derive(Clone, Debug)]
pub enum Types{
    Strobj = 0,
    Intobj = 1
}
#[derive(Clone, Debug)]
pub struct Interpreter{
    pub stack: Vec<StackTypes>,
    pub vars: HashMap<String,StackTypes>,
    pub functions: HashMap<StackTypes, String>
}
impl Interpreter{
    pub fn new() -> Self{
        Interpreter { stack: vec![], vars: HashMap::new(), functions: HashMap::new() }
    }
    pub fn push_int(mut i: Self, value: i32) -> Self{
        i.stack.push(StackTypes::new_int(value));
        i
    }
    pub fn push_str(mut i: Self, value: String) -> Self{
        i.stack.push(StackTypes::new_str(value));
        i
    }
    pub fn new_var(mut i: Self, name: String, value: StackTypes) -> Self {
        i.vars.insert(name, value);
        i
    }
}



pub fn run(mut i: Interpreter, program: &str) -> Interpreter {
    let parse = program.split(' ');
    let mut length: usize = 0;
    let mut isstr: bool = false;
    let mut strbuf: String = "".to_string();
    let mut isfn: bool = false;
    let mut currentfn: String = "".to_string();
    for _ in parse.clone(){
        length+=1;
    }
    let mut iter: usize = 0;
    while iter < length{
        let tok = parse.clone().nth(iter).unwrap();
        let int: i32;
        let isint;
        if isfn{
            if tok == "endfn"{
                i.functions.insert(StackTypes {strtype: Some(currentfn.clone().to_string()), inttype: None, selected: 0}, strbuf.to_string());
                isfn = false;
                strbuf = "".to_string();
                currentfn = "".to_string();
            }
            else{
                strbuf += &(tok.to_owned() + " ");
            }
            iter+=1;
            continue
        }
        if isstr{
            if tok == "\""{
                strbuf.pop();
                spush!(i,strbuf);
                strbuf = "".to_string();
                isstr = false;
            }
            else{
                strbuf += &(tok.to_owned() + " ");
            }
            iter+=1;
            continue;
        }
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
            let t = pop!(i);
            if t.selected == Types::Intobj as i8{
                println!("{}", t.inttype.unwrap())
            }
            else if t.selected == Types::Strobj as i8{
                println!("{}", t.strtype.unwrap())
            }
            let _ = std::io::stdout().flush();
        }
        else if tok == "\""{
            isstr = true;
        }
        else if tok == "set"{
            iter+=1;
            let name = parse.clone().nth(iter).unwrap().to_string();
            let mut y = i.clone();
            let value = pop!(y);
            new_var!(y,name, value);
            i = y;
        }
        else if tok == "fn"{
            iter+=1;
            currentfn = parse.clone().nth(iter).unwrap().to_string();
            isfn = true;
        }
        else if i.clone().vars.contains_key(tok){
            let x = i.vars.get(tok).unwrap();
            i.stack.push(x.to_owned());
        }
        else{
            let x = parse.clone().nth(iter).unwrap().to_string();
            let f: StackTypes = StackTypes {strtype: Some(x), inttype: None, selected: 0};
            if i.functions.contains_key(&f){
                let y = i.clone();
                i = run(y.clone(), y.functions.get(&f).unwrap())
            }
        }
        iter+=1;
    }
    i
}