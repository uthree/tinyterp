mod compiler;
mod object;
mod parser;
mod runtime;

use crate::runtime::Runtime;
fn main() {
    let mut runtime = Runtime::new();
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        runtime.execute(buffer);
    }
}
