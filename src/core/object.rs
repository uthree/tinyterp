use crate::core::environment::Environment;
use crate::core::error::Error;
use crate::core::parser::Node;
use crate::core::parser::Position;

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<Object>),
    Hash(Vec<Object>, Vec<Object>),
    Function {
        name: Option<String>,
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
            Object::Int(i) => write!(f, "{}", i)?,
            Object::Float(v) => {
                if v.to_string().contains(".") {
                    write!(f, "{}", v)?
                } else {
                    write!(f, "{}.0", v)?
                }
            }
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
                name: _,
                args: _,
                kwargs: _,
                body: _,
                env: _,
                pos: _,
            } => write!(f, "<function>")?,
            Object::BuiltInFunction(_func) => write!(f, "<built-in function>")?,
            _ => write!(f, "<unknown object>")?,
        };
        Ok(())
    }
}

impl Object {
    pub fn remove_return(self) -> Self {
        match self {
            Object::Return(obj) => obj.remove_return(),
            _ => self,
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            Object::Bool(_) => "bool",
            Object::Float(_) => "float",
            Object::Nil => "nil",
            Object::Int(_) => "int",
            Object::Str(_) => "str",
            Object::List(_) => "list",
            Object::Hash(_, _) => "hash",
            Object::BuiltInFunction(_) => "function",
            Object::Function {
                name: _,
                args: _,
                kwargs: _,
                body: _,
                env: _,
                pos: _,
            } => "function",
            Object::Return(obj) => obj.type_name(),
        }
    }

    // for if-expression
    pub fn to_bool(self) -> bool {
        match self {
            Object::Bool(b) => b,
            Object::Nil => false,
            _ => true,
        }
    }

    // Operators
    pub fn less_than(self, other: Self, pos: Position) -> Result<Object, Error> {
        match self {
            Object::Int(l) => match other {
                Object::Int(r) => Ok(Object::Bool(l < r)),
                Object::Float(r) => Ok(Object::Bool((l as f64) < r)),
                _ => Err(Error::TypeError(
                    format!("cannot compare int < {}", other.type_name()),
                    pos,
                )),
            },
            Object::Float(l) => match other {
                Object::Int(r) => Ok(Object::Bool(l < r as f64)),
                Object::Float(r) => Ok(Object::Bool(l < r)),
                _ => Err(Error::TypeError(
                    format!("cannot compare float < {}", other.type_name()),
                    pos,
                )),
            },
            _ => Err(Error::TypeError(
                format!(
                    "cannot compare {} < {}",
                    self.type_name(),
                    other.type_name()
                ),
                pos,
            )),
        }
    }

    pub fn get_attribute(self, index: Self, pos: Position) -> Result<Object, Error> {
        match self {
            Object::List(l) => match index {
                Object::Int(i) => {
                    let mut idx = i as usize;
                    if i < 0 {
                        idx = (-i) as usize;
                    }
                    if (i as usize) < l.len() {
                        Ok(l[idx as usize].clone())
                    } else {
                        Err(Error::IndexOutOfRange(
                            format!("index {} is out of range", i.to_string()),
                            pos,
                        ))
                    }
                }
                _ => Err(Error::TypeError(
                    format!("index must be int, given {}", index.type_name()),
                    pos,
                )),
            },
            Object::Str(s) => match index {
                Object::Int(i) => {
                    let mut idx = i as usize;
                    if i < 0 {
                        idx = (-i) as usize;
                    }
                    if (i as usize) < s.len() {
                        Ok(Object::Str(
                            s.chars().collect::<Vec<char>>()[i as usize].to_string(),
                        ))
                    } else {
                        Err(Error::IndexOutOfRange(
                            format!("index {} is out of range", i.to_string()),
                            pos,
                        ))
                    }
                }
                _ => Err(Error::TypeError(
                    format!("index must be int, given {}", index.type_name()),
                    pos,
                )),
            },
            _ => Err(Error::TypeError(
                format!(
                    "cannot calculate {}[{}]",
                    self.type_name(),
                    index.type_name()
                ),
                pos,
            )),
        }
    }

    pub fn less_than_eq(self, other: Self, pos: Position) -> Result<Object, Error> {
        match self {
            Object::Int(l) => match other {
                Object::Int(r) => Ok(Object::Bool(l <= r)),
                Object::Float(r) => Ok(Object::Bool((l as f64) <= r)),
                _ => Err(Error::TypeError(
                    format!("cannot compare int <= {}", other.type_name()),
                    pos,
                )),
            },
            Object::Float(l) => match other {
                Object::Int(r) => Ok(Object::Bool(l <= r as f64)),
                Object::Float(r) => Ok(Object::Bool(l <= r)),
                _ => Err(Error::TypeError(
                    format!("cannot compare float <= {}", other.type_name()),
                    pos,
                )),
            },
            _ => Err(Error::TypeError(
                format!(
                    "cannot compare {} <= {}",
                    self.type_name(),
                    other.type_name()
                ),
                pos,
            )),
        }
    }

    pub fn greater_than(self, other: Self, pos: Position) -> Result<Object, Error> {
        match self {
            Object::Int(l) => match other {
                Object::Int(r) => Ok(Object::Bool(l > r)),
                Object::Float(r) => Ok(Object::Bool((l as f64) > r)),
                _ => Err(Error::TypeError(
                    format!("cannot compare int > {}", other.type_name()),
                    pos,
                )),
            },
            Object::Float(l) => match other {
                Object::Int(r) => Ok(Object::Bool(l > r as f64)),
                Object::Float(r) => Ok(Object::Bool(l > r)),
                _ => Err(Error::TypeError(
                    format!("cannot compare float > {}", other.type_name()),
                    pos,
                )),
            },
            _ => Err(Error::TypeError(
                format!(
                    "cannot compare {} > {}",
                    self.type_name(),
                    other.type_name()
                ),
                pos,
            )),
        }
    }

    pub fn greater_than_eq(self, other: Self, pos: Position) -> Result<Object, Error> {
        match self {
            Object::Int(l) => match other {
                Object::Int(r) => Ok(Object::Bool(l >= r)),
                Object::Float(r) => Ok(Object::Bool((l as f64) >= r)),
                _ => Err(Error::TypeError(
                    format!("cannot compare int >= {}", other.type_name()),
                    pos,
                )),
            },
            Object::Float(l) => match other {
                Object::Int(r) => Ok(Object::Bool(l >= r as f64)),
                Object::Float(r) => Ok(Object::Bool(l >= r)),
                _ => Err(Error::TypeError(
                    format!("cannot compare float >= {}", other.type_name()),
                    pos,
                )),
            },
            _ => Err(Error::TypeError(
                format!(
                    "cannot compare {} >= {}",
                    self.type_name(),
                    other.type_name()
                ),
                pos,
            )),
        }
    }

    pub fn neg(self, pos: Position) -> Result<Object, Error> {
        match self {
            Object::Int(v) => Ok(Object::Int(-v)),
            Object::Float(v) => Ok(Object::Float(-v)),
            _ => Err(Error::TypeError(
                format!("cannot make negative {}", self.type_name()),
                pos,
            )),
        }
    }

    pub fn div(self, other: Self, pos: Position) -> Result<Object, Error> {
        match other {
            Object::Int(0i64) | Object::Float(0.0f64) => {
                return Err(Error::DivideByZero("divide by zero".to_string(), pos))
            }
            _ => {}
        }
        match self {
            Object::Int(l) => match other {
                Object::Int(r) => Ok(Object::Int(l / r)),
                Object::Float(r) => Ok(Object::Float(l as f64 / r)),
                _ => Err(Error::TypeError(
                    format!("cannot divide int / {}", other.type_name()),
                    pos,
                )),
            },
            Object::Float(l) => match other {
                Object::Int(r) => Ok(Object::Float(l / r as f64)),
                Object::Float(r) => Ok(Object::Float(l / r)),
                _ => Err(Error::TypeError(
                    format!("cannot divide float / {}", other.type_name()),
                    pos,
                )),
            },
            _ => Err(Error::TypeError(
                format!(
                    "cannot multiply {} / {}",
                    self.type_name(),
                    other.type_name()
                ),
                pos,
            )),
        }
    }

    pub fn pow(self, other: Self, pos: Position) -> Result<Object, Error> {
        match self {
            Object::Int(l) => match other {
                Object::Int(r) => Ok(Object::Int(l.pow(r as u32))),
                Object::Float(r) => Ok(Object::Float((l as f64).powf(r))),
                _ => Err(Error::TypeError(
                    format!("cannot multiply int * {}", other.type_name()),
                    pos,
                )),
            },
            Object::Float(l) => match other {
                Object::Int(r) => Ok(Object::Float(l.powf(r as f64))),
                Object::Float(r) => Ok(Object::Float(l.powf(r))),
                _ => Err(Error::TypeError(
                    format!("cannot multiply float * {}", other.type_name()),
                    pos,
                )),
            },
            _ => Err(Error::TypeError(
                format!(
                    "cannot multiply {} * {}",
                    self.type_name(),
                    other.type_name()
                ),
                pos,
            )),
        }
    }

    pub fn mul(self, other: Self, pos: Position) -> Result<Object, Error> {
        match self {
            Object::Int(l) => match other {
                Object::Int(r) => Ok(Object::Int(l * r)),
                Object::Float(r) => Ok(Object::Float(l as f64 * r)),
                _ => Err(Error::TypeError(
                    format!("cannot multiply int * {}", other.type_name()),
                    pos,
                )),
            },
            Object::Float(l) => match other {
                Object::Int(r) => Ok(Object::Float(l * r as f64)),
                Object::Float(r) => Ok(Object::Float(l * r)),
                _ => Err(Error::TypeError(
                    format!("cannot multiply float * {}", other.type_name()),
                    pos,
                )),
            },
            _ => Err(Error::TypeError(
                format!(
                    "cannot multiply {} * {}",
                    self.type_name(),
                    other.type_name()
                ),
                pos,
            )),
        }
    }

    pub fn sub(self, other: Self, pos: Position) -> Result<Object, Error> {
        match self {
            Object::Int(l) => match other {
                Object::Int(r) => Ok(Object::Int(l - r)),
                Object::Float(r) => Ok(Object::Float(l as f64 - r)),
                _ => Err(Error::TypeError(
                    format!("cannot subtract int - {}", other.type_name()),
                    pos,
                )),
            },
            Object::Float(l) => match other {
                Object::Int(r) => Ok(Object::Float(l - r as f64)),
                Object::Float(r) => Ok(Object::Float(l - r)),
                _ => Err(Error::TypeError(
                    format!("cannot subtract float - {}", other.type_name()),
                    pos,
                )),
            },
            _ => Err(Error::TypeError(
                format!(
                    "cannot subtract {} - {}",
                    self.type_name(),
                    other.type_name()
                ),
                pos,
            )),
        }
    }

    pub fn add(self, other: Self, pos: Position) -> Result<Object, Error> {
        match self {
            Object::Int(l) => match other {
                Object::Int(r) => Ok(Object::Int(l + r)),
                Object::Float(r) => Ok(Object::Float(l as f64 + r)),
                _ => Err(Error::TypeError(
                    format!("cannot add int + {}", other.type_name()),
                    pos,
                )),
            },
            Object::Float(l) => match other {
                Object::Int(r) => Ok(Object::Float(l + r as f64)),
                Object::Float(r) => Ok(Object::Float(l + r)),
                _ => Err(Error::TypeError(
                    format!("cannot add float + {}", other.type_name()),
                    pos,
                )),
            },
            Object::Str(l) => match other {
                // concatenate str
                Object::Str(r) => Ok(Object::Str(format!("{}{}", l, r))),
                _ => Err(Error::TypeError(
                    format!("cannot add str + {}", other.type_name()),
                    pos,
                )),
            },
            Object::List(l) => match other {
                // concatenate str
                Object::List(r) => {
                    let mut out = vec![];
                    out.append(&mut l.to_vec());
                    out.append(&mut r.to_vec());
                    Ok(Object::List(out))
                }
                _ => Err(Error::TypeError(
                    format!("cannot add list + {}", other.type_name()),
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
