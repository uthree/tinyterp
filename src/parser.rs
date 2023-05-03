use peg::parser;

pub enum Node {
    IntegerLiteral(String),
    FloatLiteral(String),
    Add(Box<Node>, Box<Node>),
}

peg::parser! {
    grammar tinyterp() for str {
        pub rule program() -> Node
            = expression()

        pub rule expression() -> Node
            = precedence! {
                l:(@) "+" r:@ {Node::Add(Box::new(l), Box::new(r))}
                --
                i: integer_literal() {i}
            }

        pub rule integer_literal() -> Node
            = n:$(['0'..='9']+) {Node::IntegerLiteral(String::from(n))}
    }
}
