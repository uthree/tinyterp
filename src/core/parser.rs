use peg::parser;

peg::parser! {
    pub grammar tinyterp() for str {
        pub rule program() -> Node
            = expression()

        rule ident() -> &'input str = $(['a'..='z']+)

        // ignore tokens
        rule _ = [' ' | '\t']*

        rule expression() -> Node = precedence!{
            start:position!() node:@ end:position!() { Node { start, node, end} }
            --
            x:(@) "+" y:@ { Op::Add(Box::new(x), Box::new(y)) }
            --
            x:(@) "*" y:@ { Op::Mul(Box::new(x), Box::new(y)) }
            --
            i:ident() [' '|'\n']* { Op::Ident(i.to_owned()) }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    node: Op,
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Op {
    Ident(String),
    Add(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
}
