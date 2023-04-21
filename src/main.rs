mod parser;

use parser::tinyterp::program as parse;
fn main() {
    println!("{:?}", parse("a, b = 1, 2"));
}
