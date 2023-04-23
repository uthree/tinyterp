mod object;
mod parser;
mod runtime;

use parser::tinyterp::program as parse;
fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("{:?}", parse(&input));
    }
}
