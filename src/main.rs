mod core;

use crate::core::runtime::Runtime;
fn main() {
    let mut rt = Runtime::new();
    loop {
        let mut inst = String::new();
        std::io::stdin().read_line(&mut inst).unwrap();
        println!("{:?}", rt.execute(&inst));
    }
}
