use crate::core::parser::Position;

#[derive(Debug)]
pub enum Error {
    VariableNotInitialized,
    InvaildNumberOfArguments,
    FailedToConvertType,
}
