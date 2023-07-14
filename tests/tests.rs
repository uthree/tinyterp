use tinyterp::Runtime;

pub fn run(code: &str) -> String {
    let mut rt = Runtime::new();
    tinyterp::builtin_functions::load_builtin_stdio(&mut rt.env);
    rt.evaluate(code).unwrap().to_string()
}

#[test]
fn nil() {
    assert_eq!(run(""), "nil");
    assert_eq!(run("nil"), "nil");
    assert_eq!(run("()"), "nil");
}

#[test]
fn block_return() {
    assert_eq!(run("{1; return 2; 3}"), "2");
    assert_eq!(run("{1; return {2; return 3; 4}; 5}"), "3");
}

#[test]
fn arithmetic() {
    assert_eq!(run("1+1"), "2");
    assert_eq!(run("1+1+1"), "3");
    assert_eq!(run("1+2*3"), "7");
    assert_eq!(run("3-4+5-2"), "2");
    assert_eq!(run("4**3"), "64");
    assert_eq!(run("5.0 / 2"), "2.5");
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

#[test]
fn fibonacchi() {
    run(include_str!("../examples/fibonacchi.tinyterp"));
}

#[test]
fn fizzbuzz() {
    run(include_str!("../examples/fizzbuzz.tinyterp"));
}
