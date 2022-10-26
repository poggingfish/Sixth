use std::fs::File;
use std::io::Write;

pub fn initialize_file(mut out: File){
    writeln!(out,"#include <stdio.h>").unwrap();
}