use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;

mod lexer;
mod parser;

fn main() -> io::Result<()> {
    // Grab CL args: first argument is which file to assemble, second is name of output file.
    let args: Vec<String> = env::args().collect();

    // Instantiate the input file object.
    let mut i_file = File::open(&args[1])?;
    let mut contents = String::new();
    i_file.read_to_string(&mut contents)?;

    // Lexing.
    Ok(())
}
