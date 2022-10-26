use super::codegen::{ initialize_file };
use std::fs::File;
pub fn compile(){
    let mut out = File::create("out.c").unwrap();
    initialize_file(out);
}