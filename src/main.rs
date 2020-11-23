use crate::engine::rpn;
use std::io;
use std::io::prelude::*;
pub mod engine;

fn main() {
    //test("10 20 5 / +");
    test("4 5 2 MAX");

    pause();
    return ();
}

fn test(sample: &str){
    let result = rpn(sample);
    println!("Input sample {}", sample);
    println!("Result {}", result);
}

fn pause(){
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
