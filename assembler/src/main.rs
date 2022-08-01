mod codegen;
mod tokenizer;

use std::env;
use std::fs::*;
use std::io::*;
use std::path::*;

fn read_file(path: &str) -> String {
    let mut f = File::open(&Path::new(path)).unwrap();
    let mut result = String::new();

    f.read_to_string(&mut result).unwrap();

    result
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        println!("usage: {} [file]", args[0]);
    } else {
        let s = read_file(&args[1]);
        let b = codegen::gen_code_from(tokenizer::Token::tokenizer(&s));
        print!("{{");
        for i in b {
            print!("0x{:02X},", i);
        }
        println!("}}");
    }
}
