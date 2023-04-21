mod parser;

use parser::tinyterp::program as parse;
fn main() {
    println!("{:?}", parse("if 1<2 2"));
}
