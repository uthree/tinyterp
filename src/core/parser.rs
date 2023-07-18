use peg;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub begin: usize,
    pub end: usize,
}

impl Position {
    pub fn new(begin: usize, end: usize) -> Self {
        Position { begin, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    // Literals
    IntegerLiteral(i64, Position),
    FloatLiteral(f64, Position),
    StringLiteral(String, Position),
    Nil(Position),
    Bool(bool, Position),
    List(Vec<Node>, Position),
    Hash(Vec<(Node, Node)>, Position),

    // Operators
    Neg(Box<Node>, Position),
    Add(Box<Node>, Box<Node>, Position),
    Sub(Box<Node>, Box<Node>, Position),
    Mul(Box<Node>, Box<Node>, Position),
    Div(Box<Node>, Box<Node>, Position),
    Pow(Box<Node>, Box<Node>, Position),

    // Compare Operator
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
        sequence: Box<Node>,
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

    Assign(Vec<Node>, Vec<Node>, Position),
    IfElse(Box<Node>, Box<Node>, Box<Node>, Position),
    Return(Box<Node>, Position),
    Drop(Vec<String>, Position),
    Loop(Vec<Node>, Position),
}

const RESERVED_WORDS: [&str; 11] = [
    "if", "then", "else", "not", "and", "or", "return", "nil", "true", "false", "drop",
];

peg::parser! {
    pub grammar tinyterp() for str {
        rule _ = ignore()
        rule ignore() = comment()
            / [' ' | '\t']*
        rule comment() = ("#" ([^'\n'])* "\n")
        rule newline() = (ignore()? [';' | '\n']+ ignore()?)+

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
        rule keyword_nil() = "nil"
        rule keyword_true() = "true"
        rule keyword_false() = "false"
        rule keyword_loop() = "loop"
        rule keyword_drop() = "drop"

        // Literals
        #[cache_left_rec]
        rule integer_literal() -> Node
            = begin:position!() sign:$("-"?) value:$(['0'..='9']+) end:position!() {?
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
            = begin:position!() sign:$("-"?) value_1:$(['0'..='9']+) period() value_2:$(['0'..='9']+) end:position!() {?
                let value = format!("{}{}.{}", sign, value_1, value_2).parse::<f64>();
                if value.is_ok() {
                    Ok(Node::FloatLiteral(value.unwrap(), Position::new(begin, end)))
                }
                else {
                    Err("failed to parse float")
                }
            }

        // Nil
        #[cache_left_rec]
        rule nil_literal() -> Node
            = begin:position!()  keyword_nil() end:position!() {
                Node::Nil(Position::new(begin, end))
            }
            / begin:position!()  left_paren() _ right_paren() end:position!() {
                Node::Nil(Position::new(begin, end))
            }

        // Boolean
        #[cache_left_rec]
        rule bool_literal() -> Node
            = begin:position!() keyword_true() end:position!() {
                Node::Bool(true, Position::new(begin, end))
            }
            / begin:position!() keyword_false() end:position!() {
                Node::Bool(false, Position::new(begin, end))
            }

        // ientifier
        #[cache_left_rec]
        rule identifier() -> Node
            = begin:position!() name_initial:$(['a'..='z' | 'A'..='Z' | '_']) name:$(['a'..='z' | 'A'..='Z' | '_' | '0'..='9']*) end:position!() {?
                let name = format!("{}{}", name_initial, name);
                if RESERVED_WORDS.iter().any(|w| **w == name) {
                    Err("that name is reserved.")
                }
                else {
                    Ok(Node::Identifier(name, Position::new(begin, end)))
                }
            }

        // Statements
        #[cache_left_rec]
        pub rule program() -> Node
            = _ begin:position!() _ newline()? _ seq:(sequence() ** newline()) _ newline()? _ end:position!() {
                Node::Sequence(seq, Position::new(begin, end))
            }

        #[cache_left_rec]
        rule escaped_char() -> char
            = "\\\\" {
                '\\'
            }
            / "\\\"" {
                '\"'
            }
            / "\\n" {
                '\n'
            }
            / "\\" c:[^'\"'] {
                c
            }
            / c:[^'\"'] {
                c
            }

        #[cache_left_rec]
        rule parse_string() -> String
            = chars:escaped_char()* {
                chars.iter().collect()
            }

        #[cache_left_rec]
        rule string_literal() -> Node
            = begin:position!() "\"" s:parse_string() "\"" end:position!() {
                Node::StringLiteral(s, Position::new(begin, end))
            }

        // Statements
        #[cache_left_rec]
        rule sequence() -> Node
            = _ begin:position!() left_brace() _ newline()? _ seq:(sequence() ** newline()) _ newline()? _ right_brace() end:position!()  {
                Node::Sequence(seq, Position::new(begin, end))
            }
            / _ begin:position!() keyword_loop() newline()? _ left_brace() _ newline()? _ seq:(sequence() ** newline()) _ newline()? _ right_brace() end:position!() {
                Node::Loop(seq, Position::new(begin, end))
            }
            / _ begin:position!() keyword_if() _ condition:sequence() newline()? _ keyword_then()? newline()? _ expr_true:sequence() newline()? _ keyword_else() _ newline()? _ expr_false:sequence() end:position!() _ {
                Node::IfElse(Box::new(condition), Box::new(expr_true), Box::new(expr_false), Position::new(begin, end))
            }
            / _ begin:position!() keyword_if() _ condition:sequence() newline()? _ keyword_then()? newline()? _ expr_true:sequence() end:position!() _ {
                Node::IfElse(Box::new(condition), Box::new(expr_true), Box::new(Node::Sequence(vec![], Position::new(begin, end))), Position::new(begin, end))
            }
            / statement()

        #[cache_left_rec]
        rule statement() -> Node
            = _ e:expression() _ {
                e
            }

        #[cache_left_rec]
        rule expression() -> Node
            = return_or_drop()

        #[cache_left_rec]
        rule return_or_drop() -> Node
            = _ begin:position!() keyword_return() _ expr:expression() end:position!() _ {
                Node::Return(Box::new(expr), Position::new(begin, end))
            }
            / _ begin:position!() keyword_return() end:position!() _ {
                Node::Return(Box::new(Node::Nil(Position::new(begin, end))), Position::new(begin, end))
            }
            / _ begin:position!() keyword_drop() _ identifiers:(identifier() ++ (_ comma() _)) _ end:position!() _ {
                let mut variable_names = vec![];
                for identifier in identifiers {
                    if let Node::Identifier(s, _) = identifier {
                        variable_names.push(s)
                    }
                    else {
                        panic!("parse error")
                    }
                }
                Node::Drop(variable_names, Position::new(begin, end))
            }
            / assign()

        // variable name for assign
        #[cache_left_rec]
        rule assign_left_elem() -> Node
            = get_attr()
            / identifier()

       // Assign
        #[cache_left_rec]
        rule assign_left() -> Vec<Node>
            = assign_left_elem() ++ (_ comma() _)

        #[cache_left_rec]
        rule assign_right() -> Vec<Node>
            = expression() ++ (_ comma() _)


        #[cache_left_rec]
        rule assign() -> Node
            = begin:position!() left:assign_left() _ equal() _ right:assign_right() end:position!() {?
                if left.len() == right.len() {
                    Ok(Node::Assign(left, right, Position::new(begin, end)))
                }
                else {
                    Err("The number on the right side and the left side must be the same.")
                }
            }
            / logical_or()

        #[cache_left_rec]
        rule logical_or() -> Node
            = begin:position!() left:logical_or() _ keyword_or() _ right:logical_and() end:position!() {
                Node::LogicalOr(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / logical_and()

        #[cache_left_rec]
        rule logical_and() -> Node
            = begin:position!() left:logical_and() _ keyword_and() _ right:logical_not() end:position!() {
                Node::LogicalAnd(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / logical_not()

        #[cache_left_rec]
        rule logical_not() -> Node
            = begin:position!() keyword_not() _ v:compare() end:position!() {
                Node::LogicalNot(Box::new(v), Position::new(begin, end))
            }
            / compare()

        // Compare
        #[cache_left_rec]
        rule compare() -> Node
            = begin:position!() left:compare() _ operator_cmp_eq() _ right:arithmetic_expression() end:position!() {
                Node::CmpEq(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / begin:position!() left:compare() _ operator_cmp_noteq() _ right:arithmetic_expression() end:position!() {
                Node::CmpNotEq(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / begin:position!() left:compare() _ operator_cmp_lt() _ right:arithmetic_expression() end:position!() {
                Node::CmpLessThan(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / begin:position!() left:compare() _ operator_cmp_lteq() _ right:arithmetic_expression() end:position!() {
                Node::CmpLessThanEq(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / begin:position!() left:compare() _ operator_cmp_gt() _ right:arithmetic_expression() end:position!() {
                Node::CmpGreaterThan(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / begin:position!() left:compare() _ operator_cmp_gteq() _ right:arithmetic_expression() end:position!() {
                Node::CmpGreaterThanEq(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / arithmetic_expression()

        // Arithmetic Expressions
        #[cache_left_rec]
        rule arithmetic_expression() -> Node
            = begin:position!() left:arithmetic_expression() _ operator_add() _ right:term() end:position!() {
                Node::Add(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / begin:position!() left:arithmetic_expression() _ operator_sub() _ right:term() end:position!() {
                Node::Sub(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / term()

        #[cache_left_rec]
        rule term() -> Node
            = begin:position!() left:term() _ operator_mul() _ right:number_with_pow() end:position!() {
                Node::Mul(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            /begin:position!() left:term() _ operator_div() _ right:number_with_pow() end:position!() {
                Node::Div(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / number_with_pow()

        #[cache_left_rec]
        rule number_with_pow() -> Node
            = begin:position!() left:number_with_pow() _ operator_pow() _ right:call_function() end:position!() {
                Node::Pow(Box::new(left), Box::new(right), Position::new(begin, end))
            }
            / begin:position!() operator_sub() _ value:call_function() end:position!() {
                Node::Neg(Box::new(value), Position::new(begin, end))
            }
            / call_function()


        // Call Function
        #[cache_left_rec]
        rule argument() -> (Option<String>, Node)
            = key:identifier() _ equal() _ value:expression() {
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
            = begin:position!() callable:call_function() left_paren() _ args:arguments() _ right_paren() end:position!() {
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
            = begin:position!() reciever:get_attr() _ period() _ attribute_name:identifier() end:position!() {
                let mut attr_name: String;
                if let Node::Identifier(name, _) = attribute_name {
                    attr_name = name;
                }
                else {
                    panic!("parse error")
                }
                Node::GetAttribute(Box::new(reciever), Box::new(Node::StringLiteral(attr_name, Position::new(begin, end))), Position::new(begin, end))
            }
            / begin:position!() reciever:get_attr() _ left_bracket() _ expr:expression()  _ right_bracket() _ end:position!() {
                Node::GetAttribute(Box::new(reciever), Box::new(expr), Position::new(begin, end))
            }
            / begin:position!() reciever:get_attr() _ left_bracket() _ elements:(expression() ** (_ comma() _))  _ right_bracket() _ end:position!() {
                Node::GetAttribute(Box::new(reciever), Box::new(Node::List(elements, Position::new(begin, end))), Position::new(begin, end))
            }
            / hash()

        #[cache_left_rec]
        rule hash_element() -> (Node, Node)
            = _ key:expression() _ right_arrow() _ value:expression() {
                (key, value)
            }

        #[cache_left_rec]
        rule hash() -> Node
            = begin:position!() left_brace() _ elements:(hash_element() ** (_ comma() _)) _ comma()* _ right_brace() end:position!() {
                let mut out = vec![];
                for (k ,v) in elements {
                    out.push((k, v));
                }
                Node::Hash(out, Position::new(begin, end))
            }
            / list()

        #[cache_left_rec]
        rule list() -> Node
            = begin:position!() left_bracket() elements:(expression() ** (_ comma()  _)) _ comma()*  right_bracket()  end:position!() {
                Node::List(elements, Position::new(begin, end))
            }
            / function()

        // Function Literal
        #[cache_left_rec]
        rule argument_signature() -> (String, Option<Node>)
            = key:identifier() _ equal() _ value:(expression()) {
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
            = _ begin:position!() left_paren() _ args:arguments_signature() _ right_paren() _ right_arrow() _ seq:expression() end:position!() _ {
                let (args, kwargs) = args;
                Node::Function {
                    arguments: args,
                    keyword_arguments: kwargs,
                    sequence: Box::new(seq),
                    position: Position::new(begin, end)
                }
            }
            / atom()

        #[cache_left_rec]
        rule atom() -> Node
            = float_literal()
            / integer_literal()
            / string_literal()
            / nil_literal()
            / bool_literal()
            / identifier()
            / begin:position!() left_paren() _ seq:sequence() _ right_paren() end:position!() {
                seq
            }
            / sequence()
    }
}
