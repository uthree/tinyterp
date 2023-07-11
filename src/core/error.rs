use core::fmt;

use crate::core::parser::Position;

#[derive(Debug)]
pub enum Error {
    VariableNotInitialized(String, Position),
    TypeError(String, Position),
    FailedToConvertType,
    ArgumentError(String, Position),
}
