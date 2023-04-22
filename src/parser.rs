use peg;

#[derive(Clone, Debug)]
pub enum Node {
    BoolLiteral(bool),
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
    LogicalNot(Box<Node>),
    LogicalAnd(Box<Node>, Box<Node>),
    LogicalOr(Box<Node>, Box<Node>),
    List(Vec<Node>),
    Assign(Vec<Node>, Vec<Node>),
    Identifier(String),
    If(Box<Node>, Box<Node>),
    IfElse(Box<Node>, Box<Node>, Box<Node>),
    While(Box<Node>),
    Sequence(Vec<Node>),
}

peg::parser!(pub grammar tinyterp() for str {
    rule _ = [' ' | '\n']*
    rule newline() = ['\n' | ';']+
    #[cache_left_rec]
    pub rule program() -> Node
        = stmts:statements() {
            Node::Sequence(stmts)
        }

    #[cache_left_rec]
    rule sequence() -> Node
        = _ "{" _ stmts:statements() _ "}" _ {
            Node::Sequence(stmts)
        }
        / _ "{" _ "}" _ {
            Node::Sequence(vec![])
        }

    #[cache_left_rec]
    rule statements() -> Vec<Node>
        = _ l:statements() _ newline()+ _ r:statement() _ {
            let mut out = l.clone(); out.push(r); out
        }
        / i:statement() { vec![i] }

    #[cache_left_rec]
    rule statement() -> Node
        = ls:identifiers() _ "=" _ rs:expressions() {
            Node::Assign(ls, rs)
        }
        / "if" _ cond:expression() _ "then" _ expr1:expression() _ "else" _ expr2:expression() {
            Node::IfElse(Box::new(cond), Box::new(expr1), Box::new(expr2))
        }
        / "if" _ cond:expression() _ "then" _ expr:expression() {
            Node::If(Box::new(cond), Box::new(expr))
        }
        / expression()

    #[cache_left_rec]
    rule expressions() -> Vec<Node>
        = l:expressions() _ "," _ r:expression() {
            let mut out = l.clone(); out.push(r); out
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
        l:(@) _ "and" _ r:@ { Node::LogicalAnd(Box::new(l), Box::new(r)) }
        l:(@) _ "or" _ r:@ { Node::LogicalOr(Box::new(l), Box::new(r)) }
        "not" _ e:@ { Node::LogicalNot(Box::new(e))}
        --
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
        i:identifier() { i }
        --
        _ "(" _  e:expression() _ ")" _ { e }
        _ s:sequence() _ { s }
    }
    rule integer_literal() -> Node
        = _ v:$(['0'..='9']+) _ {
            Node::IntegerLiteral(v.to_string())
        }
    rule float_literal() -> Node
        = _ v1:$(['0'..='9']+) "." v2:$(['0'..='9']+) _ {
            Node::FloatLiteral(v1.to_string() + "." + v2)
        }
    #[cache_left_rec]
    rule identifier() -> Node
        = _ v:['a'..='z' | 'A'..='Z']+ _ {
            let ident = v.iter().collect();
            if ident == "true".to_string() {
                return Node::BoolLiteral(true)
            }
            if ident == "false".to_string() {
                return Node::BoolLiteral(false)
            }
            return Node::Identifier(ident)
        }
});
