mod builtin_functions;
mod core;

use std::fs::File;
use std::io::prelude::*;

use crate::builtin_functions::load_builtin_stdio;
use crate::core::runtime::Runtime;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // start REPL
        let mut rt = Runtime::new();
        load_builtin_stdio(&mut rt.env);

        loop {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            if buffer == "\n" {
                continue;
            }
            let result = rt.evaluate(&buffer);
            if let Ok(output) = result {
                println!("-> {}", output);
            } else {
                println!("Error: {}", result.err().unwrap());
            }
        }
    } else {
        // run file
        let file_name = args[1].clone();
        let mut f = File::open(file_name).expect("file not found");
        let mut content = String::new();
        f.read_to_string(&mut content)
            .expect("something went wrong reading the file");

        let mut rt = Runtime::new();
        load_builtin_stdio(&mut rt.env);

        let result = rt.evaluate(&content);
        if let Err(err) = result {
            println!("Error: {}", err);
        }
    }
}
