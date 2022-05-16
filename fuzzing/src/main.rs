use bnf::Grammar;
use std::env;
use std::fs;

fn main() {
    // println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let grammar: Grammar = contents.parse().unwrap();
    let sentence = grammar.generate().unwrap();
    println!("{}", sentence);
}
