use peg::parser;

#[derive(Debug)]
pub enum Node {
    IntegerLiteral(String),
    FloatLiteral(String),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
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
                l:(@) _ "*" _ r:@ {Node::Mul(Box::new(l), Box::new(r))}
                l:(@) _ "/" _ r:@ {Node::Div(Box::new(l), Box::new(r))}
                --
                f: float_literal() {f}
                --
                i: integer_literal() {i}
            }

        rule integer_literal() -> Node
            = _ s:"-"? _ n:$(['0'..='9']+) _ {
                if s.is_some() {
                    Node::IntegerLiteral(String::from("-".to_string() + n))}
                        else {
                    Node::IntegerLiteral(String::from(n))
                }
            }
        rule float_literal() -> Node
            = _ s:"-"? _ n1:$(['0'..='9']+) "." n2:$(['0'..='9']+) _ {
                if s.is_some() {
                    Node::FloatLiteral(String::from("-".to_string() + n1 + "." + n2 ))}
                        else {
                    Node::FloatLiteral(String::from(n1.to_string() + "." + n2))
                }
            }
    }
}
