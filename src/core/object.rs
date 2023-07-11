use crate::core::error::Error;
use crate::core::parser::Node;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Object {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<usize>),
    HashMap(BTreeMap<usize, usize>),
    Function(Node),
    Return(Box<Object>),
}

impl Object {
    pub fn add(self, other: Self) -> Result<Object, Error> {
        match self {
            Object::Int(l) => match other {
                Object::Int(r) => Ok(Object::Int(l + r)),
                _ => {
                    unreachable!()
                }
            },
            _ => {
                unreachable!()
            }
        }
    }
}
