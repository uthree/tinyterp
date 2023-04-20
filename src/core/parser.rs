use peg::parser;

#[derive(Debug, Clone)]
pub enum Node {
    Identifier(String),
    IntegerLiteral(String),
    Assign(Box<Node>, Box<Node>),
    OperatorAdd(Box<Node>, Box<Node>),
    OperatorSub(Box<Node>, Box<Node>),
    OperatorMul(Box<Node>, Box<Node>),
    OperatorDiv(Box<Node>, Box<Node>),

    Variables(Vec<Node>),           // Vector of Node
    Variable(Box<Node>, Box<Node>), // Key, Value
    NoKey,
}

peg::parser! {
    pub grammar tinyterp() for str {
        rule _ = [' ' | '\n']*

        pub rule program() -> Node
            = assign()
            / expression()

        #[cache_left_rec]
        rule assign() -> Node
        = _ l:identifier() _ "=" _ r:expression() _ {
            Node::Assign(Box::new(l), Box::new(r))
        }

        #[cache_left_rec]
        rule expression() -> Node
            = _ l:expression() _ "+" r:term() _ {
                Node::OperatorAdd(
                    Box::new(l), Box::new(r))
            }
            / _ l:expression() _ "-" _ r:term() _ {
                Node::OperatorSub(
                    Box::new(l), Box::new(r))
            }
            / term()

        #[cache_left_rec]
        rule term() -> Node
            = _ l:term() _ "*" _ r:atom() _ {
                Node::OperatorMul(
                    Box::new(l), Box::new(r))
            }
            / _ l:term() _ "/" _ r:atom() _ {
                Node::OperatorDiv(
                    Box::new(l), Box::new(r))
            }
            / atom()

        #[cache_left_rec]
        rule atom() -> Node
            = identifier()
            / integer_literal()
            / _ "(" _ e:expression() _ ")" _ { e }

        rule identifier() -> Node
            = _ name:$(['a'..='z']+) _ { Node::Identifier(name.to_string())}

        rule integer_literal() -> Node
            = _ v:$(['0'..='9']+) _ {
                Node::IntegerLiteral(v.to_string())
            }
    }
}
