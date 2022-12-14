use rustyline::config::Configurer;

use crate::interpreter::{Interpreter, run};


pub fn repl(){
    let mut rl = rustyline::Editor::<()>::new().unwrap();
    let mut i: Interpreter = Interpreter::new();
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
        if !i.clone().stack.is_empty(){
            print!("\x08");
        }
        println!(")");
        rl.set_auto_add_history(true);
        let readline = rl.readline(" -> ").unwrap();
        i = run(i, readline.as_str(), true);
    }
}