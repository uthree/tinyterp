use core::fmt;

use crate::core::parser::Position;

#[derive(Debug)]
pub enum Error {
    VariableNotInitialized(String, Position),
    TypeError(String, Position),
    InvaildNumberOfArguments,
    FailedToConvertType,
    ArgumentError(String, Position),
    KeywordNotFound(String, Position),
}
