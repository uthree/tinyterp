use peg;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub begin: usize,
    pub end: usize,
}

impl Position {
    pub fn new(begin: usize, end: usize) -> Self {
        Position { begin, end }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    // Literals
    IntegerLiteral(i64, Position),
    FloatLiteral(f64, Position),
    StringLiteral(String, Position),

    // Operators
    Neg(Box<Node>, Position),
    Add(Box<Node>, Box<Node>, Position),
    Sub(Box<Node>, Box<Node>, Position),
    Mul(Box<Node>, Box<Node>, Position),
    Div(Box<Node>, Box<Node>, Position),
    Pow(Box<Node>, Box<Node>, Position),

    // Compare Operators
    CmpLessThan(Box<Node>, Box<Node>, Position),
    CmpGreaterThan(Box<Node>, Box<Node>, Position),
    CmpLessThanEq(Box<Node>, Box<Node>, Position),
    CmpGreaterThanEq(Box<Node>, Box<Node>, Position),
    CmpEq(Box<Node>, Box<Node>, Position),
    CmpNotEq(Box<Node>, Box<Node>, Position),

    // Logical Operators
    LogicalNot(Box<Node>, Position),
    LogicalOr(Box<Node>, Box<Node>, Position),
    LogicalAnd(Box<Node>, Box<Node>, Position),

    // Identifier
    Identifier(String, Position),

    // Sequence
    Sequence(Vec<Node>, Position),

    // define function
    Function {
        arguments: Vec<String>,
        keyword_arguments: HashMap<String, Node>,
        sequence: Vec<Node>,
        position: Position,
    },

    // call function
    CallFunction {
        callable: Box<Node>,
        arguments: Vec<Node>,
        keyword_arguments: HashMap<String, Node>,
        position: Position,
    },

    GetAttribute(Box<Node>, Box<Node>, Position),

    Assign(Vec<String>, Vec<Node>, Position),
    If(Box<Node>, Box<Node>, Position),
    IfElse(Box<Node>, Box<Node>, Position),
    Return(Box<Node>, Position),
    Break(Box<Node>, Position),
    Loop(Vec<Node>, Position),
}

peg::parser! {
    pub grammar tinyterp() for str {
        rule newline() = ['\n' | ';']+
        rule _ = [' ' | '\n' | '\t']*

        // Tokens and keywords
        rule left_paren() = "("
        rule right_paren() = ")"
        rule left_bracket() = "["
        rule right_bracket() = "]"
        rule left_brace() = "{"
        rule right_brace() = "}"

        rule operator_add() = "+"
        rule operator_sub() = "-"
        rule operator_mul() = "*"
        rule operator_div() = "/"
        rule operator_pow() = "**"

        rule operator_cmp_eq() = "=="
        rule operator_cmp_lt() = "<"
        rule operator_cmp_gt() = ">"
        rule operator_cmp_lteq() = "<="
        rule operator_cmp_gteq() = ">="
        rule operator_cmp_noteq() = "!="

        rule comma() = ","
        rule period() = "."

        rule equal() = "="
        rule right_arrow() = "->"

        rule keyword_if() = "if"
        rule keyword_then() = "then"
        rule keyword_else() = "else"
        rule keyword_not() = "not"
        rule keyword_and() = "and"
        rule keyword_or() = "or"
        rule keyword_return() = "return"
        rule keyword_break() = "break"

        // Literals
        #[cache_left_rec]
        rule integer_literal() -> Node
            = _ begin:position!() sign:$("-"?) value:$(['0'..='9']+) end:position!() _ {?
                let value = format!("{}{}", sign, value).parse::<i64>();
                if value.is_ok() {
                    Ok(Node::IntegerLiteral(value.unwrap(), Position::new(begin, end)))
                }
                else {
                    Err("failed to parse integer")
                }
            }

        #[cache_left_rec]
        rule float_literal() -> Node
            = _ begin:position!() sign:$("-"?) value_1:$(['0'..='9']+) period() value_2:$(['0'..='9']+) end:position!() _ {?
                let value = format!("{}{}.{}", sign, value_1, value_2).parse::<f64>();
                if value.is_ok() {
                    Ok(Node::FloatLiteral(value.unwrap(), Position::new(begin, end)))
                }
                else {
                    Err("failed to parse float")
                }
            }

        // ientifier
        #[cache_left_rec]
        rule identifier() -> Node
            = _ begin:position!() name_initial:$(['a'..='z' | 'A'..='Z' | '_']) name:$(['a'..='z' | 'A'..='Z' | '_' | '0'..='9']*) end:position!() _ {
                Node::Identifier(format!("{}{}", name_initial, name), Position::new(begin, end))
            }

        // todo fix: Can't parse "\""
        #[cache_left_rec]
        rule string_literal() -> Node
            = _ begin:position!() "\"" s:$([^'\"']*) "\"" end:position!() _ {
                Node::StringLiteral(s.to_string(), Position::new(begin, end))
            }

        #[cache_left_rec]
        pub rule expression() -> Node
            = assign()

        #[cache_left_rec]
        rule variable_name() -> String
            = v:identifier() {
                match v {
                    Node::Identifier(s, _) => {
                        s
                    }
                    _ => {
                        panic!("parse error");
                    }
                }
            }

        // Assign
        #[cache_left_rec]
        rule assign_left() -> Vec<String>
            = variable_name() ++ (_ comma() _)

        #[cache_left_rec]
        rule assign_right() -> Vec<Node>
            = expression() ++ (_ comma() _)

        #[cache_left_rec]
        rule assign() -> Node
            = _ begin:position!() left:assign_left() _ equal() _ right:assign_right() end:position!() _ {?
                if left.len() == right.len() {
                    Ok(Node::Assign(left, right, Position::new(begin, end)))
                }
                else {
                    Err("The number on the right side and the left side must be the same.")
                }
            }
            / function()

        // Function Literal
        #[cache_left_rec]
        rule argument_signature() -> (String, Option<Node>)
            = _ key:identifier() _ equal() _ value:(expression()) {
                if let Node::Identifier(key_string, _) = key {
                    (key_string, Some(value))
                }
                else {
                    panic!("parse error")
                }
            }
            / _ key:identifier() !equal() {
                if let Node::Identifier(key_string, _) = key {
                    (key_string, None)
                }
                else {
                    panic!("parse error")
                }
            }

        #[cache_left_rec]
        rule arguments_signature() -> (Vec<String>, HashMap<String, Node>)
            = args:(argument_signature() ** (_ comma() _)) {
                let mut args_vec = vec![];
                let mut kwargs_hash = HashMap::new();
                for arg in args {
                    let (k, v) = arg;
                    if v.is_some() {
                        kwargs_hash.insert(k, v.unwrap());
                    }
                    else {
                        args_vec.push(k);
                    }
                }
                (args_vec, kwargs_hash)
            }

        #[cache_left_rec]
        rule function() -> Node
            = _ begin:position!() left_paren() _ args:arguments_signature() _ right_paren() _ right_arrow() seq:sequence() _ end:position!() _ {
                match seq {
                    Node::Sequence(s, _) => {
                        let seq = s;
                        let (args, kwargs) = args;
                        Node::Function {
                            arguments: args,
                            keyword_arguments: kwargs,
                            sequence: seq,
                            position: Position::new(begin, end)
                        }
                    }
                    _ => {
                        panic!("parse error")
                    }
                }
            }
            / sequence()

        // Statements
        #[cache_left_rec]
        rule sequence() -> Node
            = _ begin:position!() left_brace() _ newline()*  _  seq:(logical_or() ** (_ newline() _)) _ newline()* _ right_brace() end:position!() _ {
                Node::Sequence(seq, Position::new(begin, end))
            }
            / _ begin:position!() left_brace() _ newline()*  _ right_brace() end:position!() _ {
                Node::Sequence(vec![], Position::new(begin, end))
            }
            / logical_or()

        #[cache_left_rec]
        rule logical_or() -> Node
            = _ begin:position!() left:logical_or() _ keyword_or() _ right:logical_and() end:position!() {
                Node::LogicalOr(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / logical_and()

        #[cache_left_rec]
        rule logical_and() -> Node
            = _ begin:position!() left:logical_and() _ keyword_and() _ right:logical_not() end:position!() {
                Node::LogicalAnd(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / logical_not()

        #[cache_left_rec]
        rule logical_not() -> Node
            =  _ begin:position!() keyword_not() _ v:compare() end:position!() _ {
                Node::LogicalNot(Box::new(v), Position::new(begin, end))
            }
            / compare()

        // Compare
        #[cache_left_rec]
        rule compare() -> Node
            = _ begin:position!() left:compare() _ operator_cmp_eq() _ right:arithmetic_expression() end:position!() _ {
                Node::CmpEq(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / _ begin:position!() left:compare() _ operator_cmp_noteq() _ right:arithmetic_expression() end:position!() _ {
                Node::CmpNotEq(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / _ begin:position!() left:compare() _ operator_cmp_lt() _ right:arithmetic_expression() end:position!() _ {
                Node::CmpLessThan(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / _ begin:position!() left:compare() _ operator_cmp_lteq() _ right:arithmetic_expression() end:position!() _ {
                Node::CmpLessThanEq(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / _ begin:position!() left:compare() _ operator_cmp_gt() _ right:arithmetic_expression() end:position!() _ {
                Node::CmpGreaterThan(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / _ begin:position!() left:compare() _ operator_cmp_gteq() _ right:arithmetic_expression() end:position!() _ {
                Node::CmpGreaterThanEq(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / arithmetic_expression()

        // Arithmetic Expressions
        #[cache_left_rec]
        rule arithmetic_expression() -> Node
            = _ begin:position!() left:arithmetic_expression() _ operator_add() _ right:term() end:position!() _ {
                Node::Add(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / _ begin:position!() left:arithmetic_expression() _ operator_sub() _ right:term() end:position!() _ {
                Node::Sub(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / term()

        #[cache_left_rec]
        rule term() -> Node
            = _ begin:position!() left:term() _ operator_mul() _ right:number_with_pow() end:position!() _ {
                Node::Mul(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / _ begin:position!() left:term() _ operator_div() _ right:number_with_pow() end:position!() _ {
                Node::Div(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / number_with_pow()

        #[cache_left_rec]
        rule number_with_pow() -> Node
            = _ begin:position!() left:number_with_pow() _ operator_pow() _ right:call_function() end:position!() _ {
                Node::Pow(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / _ begin:position!() operator_sub() _ value:call_function() end:position!() _ {
                Node::Neg(Box::new(value), Position::new(begin, end))
            }
            / call_function()


        // Call Function
        #[cache_left_rec]
        rule argument() -> (Option<String>, Node)
            = _ key:identifier() _ equal() _ value:expression() {
                if let Node::Identifier(key, _) = key {
                    (Some(key), value)
                }
                else {
                    panic!("parse error")
                }
            }
            / _ value:expression() !equal() {
                (None, value)
            }

        #[cache_left_rec]
        rule arguments() -> (Vec<Node>, HashMap<String, Node>)
        = args:(argument() ** (_ comma() _)) {
            let mut h = HashMap::<String, Node>::new();
            let mut v = vec![];
            for (key, value) in args {
                if key.is_some() {
                    h.insert(key.unwrap(), value);
                }
                else {
                    v.push(value);
                }
            }
            (v, h)
        }

        #[cache_left_rec]
        rule call_function() -> Node
            = _ begin:position!() callable:call_function() left_paren() args:arguments() right_paren() end:position!() _ {
                let (v, h) = args;
                Node::CallFunction {
                    callable: Box::new(callable),
                    arguments: v,
                    keyword_arguments: h,
                    position: Position::new(begin, end)
                }
            }
            / get_attr()

        #[cache_left_rec]
        rule get_attr() -> Node
            = begin:position!() reciever:get_attr() period() attribute_name:identifier() end:position!() {
                let mut attr_name: String;
                if let Node::Identifier(name, _) = attribute_name {
                    attr_name = name;
                }
                else {
                    panic!("parse error")
                }
                Node::GetAttribute(Box::new(reciever), Box::new(Node::StringLiteral(attr_name, Position::new(begin, end))), Position::new(begin, end))
            }
            / atom()

        #[cache_left_rec]
        rule atom() -> Node
            = float_literal()
            / integer_literal()
            / string_literal()
            / identifier()
            / _ begin:position!() left_paren() _ expression:expression() _ right_paren() end:position!() _ {
                expression
            }
    }
}
