use crate::ast::Node;
use crate::ast::NodeType;

peg::parser! {
    pub grammar tinyterp() for str {
        rule expression() -> Node
            = int:integer_literal() {
                println!("EXPR INT LITERAL");
                int
            }

        rule integer_literal() -> Node
            = n:$(['0'..='9']+) {?
                println!("INTEGER LITERAL PS");
                Ok(Node::new(NodeType::IntegerLiteral, n.to_string(), vec![]))
            }


        rule add() -> Node
            = left:expression() ("+") right:expression() {?
                Ok(Node::new(NodeType::OperatorAdd, "".to_string(), vec![Box::new(left), Box::new(right)]))
            }

        rule whitespace() = quiet!{[' ' | '\t']+}

        rule newline() = quiet!{['\n']+}

        pub rule program() -> Node
            = i:integer_literal() {
                println!("{:?}", i);
                i
            }
    }
}
