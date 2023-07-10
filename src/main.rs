mod core;

use crate::core::parser::tinyterp::program as parse;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // start REPL
        loop {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            let node = parse(&buffer);
            println!("{:?}", node);
        }
    } else {
        // run file
        println!("{}", args[1]);
    }
}
