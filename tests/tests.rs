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
    assert_eq!(run("nil"), "nil");
    assert_eq!(run("()"), "nil");
}

#[test]
fn arithmetic() {
    assert_eq!(run("1+1"), "2");
    assert_eq!(run("1+1+1"), "3");
}

#[test]
fn functions() {
    assert_eq!(run("a = 3; f = (x) -> {x + a}; f(1)"), "4");
    assert_eq!(run("f = (x, a=1) -> {x + a}; f(1)"), "2");
    assert_eq!(run("f = (x, a=1) -> {x + a}; f(1, a=2)"), "3");
}

#[test]
fn list() {
    assert_eq!(run("[]"), "[]");
    assert_eq!(run("[1]"), "[1]");
    assert_eq!(run("[1, 2, 3]"), "[1, 2, 3]");
    assert_eq!(run("[1, 2, [3, 4]]"), "[1, 2, [3, 4]]");
}
