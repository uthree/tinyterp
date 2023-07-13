use crate::core::error::Error;
use crate::core::object::Object;
use crate::core::parser::Position;
use std::collections::BTreeMap;

pub fn builtin_print(
    args: Vec<Object>,
    kwargs: BTreeMap<String, Object>,
    pos: Position,
) -> Result<Object, Error> {
    // this function takes only one argument.
    if !kwargs.is_empty() || args.len() != 1 {
        Err(Error::ArgumentError(
            "function `print` takes only one argument.".to_string(),
            pos,
        ))
    } else {
        let arg = args[0].clone();
        let mut output = "".to_string();
        match arg {
            Object::Str(s) => {
                output = s;
            }
            _ => {
                output = arg.to_string();
            }
        }
        println!("{}", output);
        Ok(Object::Str(output))
    }
}
