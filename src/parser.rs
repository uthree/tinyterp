use peg;

#[derive(Clone, Debug)]
pub enum Node {
    IntegerLiteral(String),
    FloatLiteral(String),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    DoubleEqual(Box<Node>, Box<Node>),
    LessThan(Box<Node>, Box<Node>),
    GreaterThan(Box<Node>, Box<Node>),
    LessThanEqual(Box<Node>, Box<Node>),
    GreaterThanEqual(Box<Node>, Box<Node>),
    Assign(Vec<Node>, Vec<Node>),
    Identifier(String),
    If(Box<Node>, Box<Node>),
    IfElse(Box<Node>, Box<Node>),
}

peg::parser!(pub grammar tinyterp() for str {
    rule _ = [' ' | '\n']*
    #[cache_left_rec]
    pub rule program() -> Node
        = statement()

    #[cache_left_rec]
    rule statement() -> Node
        = ls:identifiers() _ "=" _ rs:expressions() {
            Node::Assign(ls, rs)
        }
        / "if" _ cond:expression() _ expr:expression() {
            Node::If(Box::new(cond), Box::new(expr))
        }
        / expression()

    #[cache_left_rec]
    rule expressions() -> Vec<Node>
        = l:expressions() _ "," _ r:expression() {
            {let mut out = l.clone(); out.push(r); out}
        }
        / i:expression() { vec![i] }

    #[cache_left_rec]
    rule identifiers() -> Vec<Node>
        = l:identifiers() _ "," _ r:identifier() {
            {let mut out = l.clone(); out.push(r); out}
        }
        / i:identifier() { vec![i] }

    rule expression() -> Node
        = operators()

    #[cache_left_rec]
    rule operators() -> Node = precedence! {
        l:(@) _ "==" _ r:@ { Node::DoubleEqual(Box::new(l), Box::new(r)) }
        l:(@) _ "<" _ r:@ { Node::LessThan(Box::new(l), Box::new(r)) }
        l:(@) _ ">" _ r:@ { Node::GreaterThan(Box::new(l), Box::new(r)) }
        l:(@) _ "<=" _ r:@ { Node::LessThanEqual(Box::new(l), Box::new(r)) }
        l:(@) _ ">=" _ r:@ { Node::GreaterThanEqual(Box::new(l), Box::new(r)) }
        --
        l:(@) _  "*" _ r:@ { Node::Mul(Box::new(l), Box::new(r)) }
        l:(@) _ "/" _ r:@ { Node::Div(Box::new(l), Box::new(r)) }
        --
        l:(@) _ "-" _ r:@ { Node::Sub(Box::new(l), Box::new(r)) }
        l:(@) _ "+" _ r:@ { Node::Add(Box::new(l), Box::new(r)) }
        --
        l:(@) _ "**" _ r:@ { Node::Pow(Box::new(l), Box::new(r))}
        --
        f:float_literal() { f }
        i:integer_literal() { i }
        --
        _ "(" _  e:expression() _ ")" _ { e }
    }
    rule integer_literal() -> Node
        = _ v:$(['0'..='9']+) _ {
            Node::IntegerLiteral(v.to_string())
        }
    rule float_literal() -> Node
        = _ v1:$(['0'..='9']+) "." v2:$(['0'..='9']+) _ {
            Node::FloatLiteral(v1.to_string() + "." + v2)
        }
    rule identifier() -> Node
        = _ v:['a'..='z' | 'A'..='Z']+ _ {
            Node::Identifier(v.iter().collect())
        }
});
