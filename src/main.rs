mod core;

use crate::core::runtime::Runtime;
fn main() {
    let mut rt = Runtime::new();
    println!("{:?}", rt.execute("1+2"));
}
