use peg::parser;

pub enum Node {
    IntegerLiteral(String),
    FloatLiteral(String),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
}

peg::parser! {
    pub grammar tinyterp() for str {
        rule _ =
            [' '|'\n']*
        pub rule program() -> Node
            = _ e:expression() _ {e}

        rule expression() -> Node
            = precedence! {
                l:(@) _ "+" _ r:@ {Node::Add(Box::new(l), Box::new(r))}
                l:(@) _ "-" _ r:@ {Node::Sub(Box::new(l), Box::new(r))}
                --
                i: integer_literal() {i}
            }

        rule integer_literal() -> Node
            = _ n:$(['0'..='9']+) _ {Node::IntegerLiteral(String::from(n))}
    }
}
