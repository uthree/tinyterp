mod builtin_functions;
mod core;

use tinyterp::Runtime;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // start REPL
        let mut rt = Runtime::new();
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
        println!("{}", args[1]);
    }
}
