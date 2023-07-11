mod builtin_functions;
pub mod core;

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
            println!("Parse Result: {:?}\n\n", node);
            if node.is_ok() {
                let output = env.evaluate_program(&node.unwrap());
                println!("Output: {:?} \n\n", output);
                if output.is_ok() {
                    println!("-> {}", output.unwrap());
                }
                //println!("{:?}", env);
            } else {
                println!("Syntax Error {:?}", &node.err().unwrap())
            }
        }
    } else {
        // run file
        println!("{}", args[1]);
    }
}
