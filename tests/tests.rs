use tinyterp::core::environment::Environment;
use tinyterp::core::parser::tinyterp::program as parse;

pub fn run(code: &str) -> String {
    let mut env = Environment::new();
    let node = parse(code);
    let output = env.evaluate_program(&node.unwrap());
    output.unwrap().to_string()
}

#[test]
fn nil() {
    assert_eq!(run(""), "nil");
}

#[test]
fn arithmetic() {
    assert_eq!(run("1+1"), "2");
}
