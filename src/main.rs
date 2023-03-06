mod ast;
mod compiler;
mod grammar;
mod runtime;

use crate::runtime::Runtime;
fn main() {
    let r = Runtime::new();
    r.execute_code("112");
}
