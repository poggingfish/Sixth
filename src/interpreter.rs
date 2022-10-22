use crate::stacktypes::*;
#[macro_export]
macro_rules! push {
    ($i: tt,$f:tt) => {
        $i = interpreter::Interpreter::push_int($i.clone(),$f)
    };
}
#[macro_export]
macro_rules! pop {
    ($i: tt) => {
        $i.stack.pop().unwrap().inttype.unwrap()
    };
}
#[macro_export]
macro_rules! add {
    ($p: tt) => {
        let x = pop!($p);
        let y = pop!($p);
        let z = x+y;
        push!($p,z);
    };
}
#[macro_export]
macro_rules! subtract {
    ($p: tt) => {
        let x = pop!($p);
        let y = pop!($p);
        let z = x-y;
        push!($p,z);
    };
}
#[macro_export]
macro_rules! times {
    ($p: tt) => {
        let x = pop!($p);
        let y = pop!($p);
        let z = x*y;
        push!($p,z);
    };
}
#[macro_export]
macro_rules! divide {
    ($p: tt) => {
        let x = pop!($p);
        let y = pop!($p);
        let z = x/y;
        push!($p,z);
    };
}
#[derive(Clone)]
pub struct Interpreter{
    pub stack: Vec<StackTypes>
}
impl Interpreter{
    pub fn new() -> Self{
        Interpreter { stack: vec![] }
    }
    pub fn push_int(mut i: Self, value: i32) -> Self{
        i.stack.push(StackTypes::new_int(value));
        i
    }
}