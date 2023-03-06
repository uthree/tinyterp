mod core;

use crate::core::parser::tinyterp::program as parse;
fn main() {
    println!("{:?}", parse("a+b+c+d*e"));
}
