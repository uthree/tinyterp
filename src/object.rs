use std::collections::HashMap;

struct TypeError {}

enum Object {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Namespace(HashMap<String, Object>),
}

impl Object {
    fn op_add(&self, other: &Object) -> Result<Object, TypeError> {
        match *self {
            Object::Int(s) => match *other {
                Object::Int(o) => return Ok(Object::Int(s + o)),
                Object::Float(o) => return Ok(Object::Float(s as f64 + o)),
                _ => Err(TypeError {}),
            },
            Object::Float(s) => match *other {
                Object::Int(o) => return Ok(Object::Float(s + o as f64)),
                Object::Float(o) => return Ok(Object::Float(s as f64 + o)),
                _ => Err(TypeError {}),
            },
            _ => Err(TypeError {}),
        }
    }

    fn op_sub(&self, other: &Object) -> Result<Object, TypeError> {
        match *self {
            Object::Int(s) => match *other {
                Object::Int(o) => return Ok(Object::Int(s - o)),
                Object::Float(o) => return Ok(Object::Float(s as f64 - o)),
                _ => Err(TypeError {}),
            },
            Object::Float(s) => match *other {
                Object::Int(o) => return Ok(Object::Float(s - o as f64)),
                Object::Float(o) => return Ok(Object::Float(s as f64 - o)),
                _ => Err(TypeError {}),
            },
            _ => Err(TypeError {}),
        }
    }

    fn op_mul(&self, other: &Object) -> Result<Object, TypeError> {
        match *self {
            Object::Int(s) => match *other {
                Object::Int(o) => return Ok(Object::Int(s * o)),
                Object::Float(o) => return Ok(Object::Float(s as f64 * o)),
                _ => Err(TypeError {}),
            },
            Object::Float(s) => match *other {
                Object::Int(o) => return Ok(Object::Float(s * o as f64)),
                Object::Float(o) => return Ok(Object::Float(s as f64 * o)),
                _ => Err(TypeError {}),
            },
            _ => Err(TypeError {}),
        }
    }

    fn op_div(&self, other: &Object) -> Result<Object, TypeError> {
        match *self {
            Object::Int(s) => match *other {
                Object::Int(o) => return Ok(Object::Int(s / o)),
                Object::Float(o) => return Ok(Object::Float(s as f64 / o)),
                _ => Err(TypeError {}),
            },
            Object::Float(s) => match *other {
                Object::Int(o) => return Ok(Object::Float(s / o as f64)),
                Object::Float(o) => return Ok(Object::Float(s as f64 / o)),
                _ => Err(TypeError {}),
            },
            _ => Err(TypeError {}),
        }
    }
}
