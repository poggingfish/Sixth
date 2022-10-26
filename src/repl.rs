use crate::interpreter::{Interpreter, run};


pub fn repl(){
    let mut rl = rustyline::Editor::<()>::new().unwrap();
    let mut i: Interpreter;
    i = Interpreter::new();
    loop{
        print!("(");
        for i in i.clone().stack{
            if i.selected == 1{
                print!("{} ", i.inttype.unwrap())
            }
            else{
                print!("\"{}\" ", i.strtype.unwrap())
            }
        }
        if i.clone().stack.len() >= 1{
            print!("\x08");
        }
        println!(")");
        let readline = rl.readline(" -> ").unwrap();
        i = run(i, readline.as_str(), true);
    }
}