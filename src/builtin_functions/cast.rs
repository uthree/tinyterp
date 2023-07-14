use crate::core::error::Error;
use crate::core::object::Object;
use crate::core::parser::Position;
use std::collections::BTreeMap;

pub fn get_type(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `type` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        Ok(Object::Str(args[0].type_name().to_string()))
    }
}

// convert any-to-str
pub fn to_str(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `str` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match &args[0] {
            Object::Str(s) => Ok(Object::Str(s.clone())),
            _ => Ok(Object::Str(args[0].to_string())),
        }
    }
}

// convert any-to-int
pub fn to_int(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `int` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Int(i)),
            Object::Float(f) => Ok(Object::Int(f as i64)),
            Object::Str(s) => {
                let parse_result = s.parse::<i64>();
                if let Ok(i) = parse_result {
                    Ok(Object::Int(i))
                } else {
                    Err(Error::ParseError(
                        format!("failed to parse `{}` as int", &args[0].type_name()),
                        pos,
                    ))
                }
            }
            _ => Err(Error::TypeError(
                format!("cannot convert {} to int", args[0].type_name()),
                pos,
            )),
        }
    }
}

// convert any-to-float
pub fn to_float(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `float` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        match args[0].clone() {
            Object::Int(i) => Ok(Object::Float(i as f64)),
            Object::Float(f) => Ok(Object::Float(f)),
            Object::Str(s) => {
                let parse_result = s.parse::<f64>();
                if let Ok(f) = parse_result {
                    Ok(Object::Float(f))
                } else {
                    Err(Error::ParseError(
                        format!("failed to parse `{}` as float", &args[0].type_name()),
                        pos,
                    ))
                }
            }
            _ => Err(Error::TypeError(
                format!("cannot convert {} to float", args[0].type_name()),
                pos,
            )),
        }
    }
}
