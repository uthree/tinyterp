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
    if kwargs.len() > 0 || args.len() != 1 {
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
    if kwargs.len() > 0 || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `str` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        todo!()
    }
}
