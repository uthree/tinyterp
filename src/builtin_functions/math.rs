use crate::core::error::Error;
use crate::core::object::Object;
use crate::core::parser::Position;
use std::collections::BTreeMap;

pub fn exp(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `exp` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float((i as f64).exp())),
            Object::Float(f) => Ok(Object::Float(f.exp())),
            _ => Err(Error::TypeError(
                format!("cannot calculate exp({})", args[0].type_name()),
                pos,
            )),
        }
    }
}

pub fn sin(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `sin` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float((i as f64).sin())),
            Object::Float(f) => Ok(Object::Float(f.sin())),
            _ => Err(Error::TypeError(
                format!("cannot calculate sin({})", args[0].type_name()),
                pos,
            )),
        }
    }
}

pub fn sinh(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `sinh` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float((i as f64).sinh())),
            Object::Float(f) => Ok(Object::Float(f.sinh())),
            _ => Err(Error::TypeError(
                format!("cannot calculate sinh({})", args[0].type_name()),
                pos,
            )),
        }
    }
}

pub fn cos(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `cos` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float((i as f64).cos())),
            Object::Float(f) => Ok(Object::Float(f.cos())),
            _ => Err(Error::TypeError(
                format!("cannot calculate cos({})", args[0].type_name()),
                pos,
            )),
        }
    }
}

pub fn cosh(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `cosh` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float((i as f64).cosh())),
            Object::Float(f) => Ok(Object::Float(f.cosh())),
            _ => Err(Error::TypeError(
                format!("cannot calculate cosh({})", args[0].type_name()),
                pos,
            )),
        }
    }
}

pub fn tan(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `tan` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float((i as f64).tan())),
            Object::Float(f) => Ok(Object::Float(f.tan())),
            _ => Err(Error::TypeError(
                format!("cannot calculate tan({})", args[0].type_name()),
                pos,
            )),
        }
    }
}

pub fn tanh(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `tanh` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float((i as f64).tanh())),
            Object::Float(f) => Ok(Object::Float(f.tanh())),
            _ => Err(Error::TypeError(
                format!("cannot calculate tanh({})", args[0].type_name()),
                pos,
            )),
        }
    }
}

pub fn asin(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `asin` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float((i as f64).asin())),
            Object::Float(f) => Ok(Object::Float(f.asin())),
            _ => Err(Error::TypeError(
                format!("cannot calculate asin({})", args[0].type_name()),
                pos,
            )),
        }
    }
}

pub fn acos(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `acos` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float((i as f64).acos())),
            Object::Float(f) => Ok(Object::Float(f.acos())),
            _ => Err(Error::TypeError(
                format!("cannot calculate acos({})", args[0].type_name()),
                pos,
            )),
        }
    }
}

pub fn atan(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `atan` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float((i as f64).atan())),
            Object::Float(f) => Ok(Object::Float(f.atan())),
            _ => Err(Error::TypeError(
                format!("cannot calculate acos({})", args[0].type_name()),
                pos,
            )),
        }
    }
}

pub fn ln(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `ln` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float((i as f64).ln())),
            Object::Float(f) => Ok(Object::Float(f.ln())),
            _ => Err(Error::TypeError(
                format!("cannot calculate ln({})", args[0].type_name()),
                pos,
            )),
        }
    }
}

pub fn sqrt(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `sqrt` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float((i as f64).sqrt())),
            Object::Float(f) => Ok(Object::Float(f.sqrt())),
            _ => Err(Error::TypeError(
                format!("cannot calculate sqrt({})", args[0].type_name()),
                pos,
            )),
        }
    }
}
