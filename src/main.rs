mod core;

use crate::core::environment::Environment;
use crate::core::parser::tinyterp::program as parse;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // start REPL
        let mut env = Environment::new();
        loop {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            let node = parse(&buffer);
            println!("{:?}", node);
            println!("{:?}", env.evaluate(&node.unwrap()));
            println!("{:?}", env);
        }
    } else {
        // run file
        println!("{}", args[1]);
    }
}
