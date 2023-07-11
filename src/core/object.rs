use crate::core::environment::Environment;
use crate::core::error::Error;
use crate::core::parser::Node;
use crate::core::parser::Position;
use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<Object>),
    Hash(BTreeMap<Object, Object>),
    Function {
        args: Vec<String>,
        kwargs: HashMap<String, Node>,
        body: Node,
        env: Environment,
        pos: Position,
    },
    BuiltInFunction(fn(Vec<Object>, BTreeMap<String, Object>, Position) -> Result<Object, Error>),
    Return(Box<Object>),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Nil => write!(f, "nil")?,
            Object::Bool(b) => {
                if *b {
                    write!(f, "true")?
                } else {
                    write!(f, "false")?
                }
            }
            Object::Int(i) => write!(f, "{}", i.to_string())?,
            Object::Float(v) => write!(f, "{}", v.to_string())?,
            Object::Str(s) => write!(f, "{:?}", s)?,
            Object::List(l) => write!(
                f,
                "[{}]",
                l.iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?,
            Object::Function {
                args,
                kwargs,
                body,
                env,
                pos,
            } => write!(f, "<function>")?,
            Object::BuiltInFunction(func) => write!(f, "<built-in function>")?,
            _ => write!(f, "<unknown object>")?,
        };
        Ok(())
    }
}

impl Object {
    pub fn type_name(&self) -> &str {
        match self {
            Object::Bool(_) => "bool",
            Object::Float(_) => "float",
            Object::Nil => "nil",
            Object::Int(_) => "int",
            Object::Str(_) => "str",
            Object::List(_) => "list",
            Object::Hash(_) => "hash",
            Object::BuiltInFunction(_) => "function",
            Object::Function {
                args,
                kwargs,
                body,
                env,
                pos,
            } => "function",
            Object::Return(obj) => obj.type_name(),
        }
    }

    // Operators

    pub fn add(self, other: Self, pos: Position) -> Result<Object, Error> {
        match self {
            Object::Int(l) => match other {
                Object::Int(r) => Ok(Object::Int(l + r)),
                Object::Float(r) => Ok(Object::Float(l as f64 + r)),
                _ => Err(Error::TypeError(
                    format!("cannot add int and {}", other.type_name()),
                    pos,
                )),
            },
            _ => Err(Error::TypeError(
                format!("cannot add {} and {}", self.type_name(), other.type_name()),
                pos,
            )),
        }
    }
}
