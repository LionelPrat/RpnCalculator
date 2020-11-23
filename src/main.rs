use crate::engine::parse_split;
use std::io;
use std::io::prelude::*;
pub mod engine;

fn main() {
    let sample = "20 5 /";

    println!("Hello, world! {}", sample);

    println!("Hello, world! {}", parse_split(sample)[0]);
    pause();
    return ();
}

fn pause(){
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
