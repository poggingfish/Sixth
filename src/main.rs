mod interpreter;
mod stacktypes;
mod repl;
mod compiler;
mod builtins;
use std::{fs::read_to_string, env::args, process::exit};
use interpreter::*;
use repl::repl;
fn main(){
    if args().len() < 2{
        repl();
        exit(0);
    }
    if args().len() > 2{
        if args().nth(1).unwrap() == "compile"{
            let _t = match read_to_string(args().nth(2).unwrap()){
                Ok(t) => t
                    .replace('\n'," ")
                    .replace("  ", " ")
                    .replace('\t'," "),
                Err(_) => panic!("File unknown")
            };
            compiler::compiler::compile();
            exit(0);
        }
    }
    let i: Interpreter = Interpreter::new();
    let t = match read_to_string(args().nth(1).unwrap()){
        Ok(t) => t
            .replace('\n'," ")
            .replace("  ", " ")
            .replace('\t'," "),
        Err(_) => panic!("File unknown")
    };
    let i = run(i, t.as_str(), true);
    let _i = run(i,"main",true);
}

#[cfg(test)]
mod tests{
    use crate::interpreter::Interpreter;
    use crate::interpreter::run;
    #[test]
    fn add(){
        let i: Interpreter;
        i = Interpreter::new();
        let mut i = run(i,"1 1 +", true);
        assert_eq!(i.stack.pop().unwrap().inttype.unwrap(),2);
    }
    #[test]
    fn sub(){
        let i: Interpreter;
        i = Interpreter::new();
        let mut i = run(i,"4 2 -", true);
        assert_eq!(i.stack.pop().unwrap().inttype.unwrap(),2);
    }
    #[test]
    fn mul(){
        let i: Interpreter;
        i = Interpreter::new();
        let mut i = run(i,"4 2 *", true);
        assert_eq!(i.stack.pop().unwrap().inttype.unwrap(),8);
    }
    #[test]
    fn div(){
        let i: Interpreter;
        i = Interpreter::new();
        let mut i = run(i,"4 2 /", true);
        assert_eq!(i.stack.pop().unwrap().inttype.unwrap(),2);
    }
    #[test]
    fn func(){
        let i: Interpreter;
        i = Interpreter::new();
        let mut i = run(i," fn add 1 + endfn 1 add", true);
        assert_eq!(i.stack.pop().unwrap().inttype.unwrap(),2);
    }
    #[test]
    fn str(){
        let i: Interpreter;
        i = Interpreter::new();
        let mut i = run(i,"\" Test \"", true);
        assert_eq!(i.stack.pop().unwrap().strtype.unwrap(),"Test");
    }
}