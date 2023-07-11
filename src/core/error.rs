#[derive(Debug)]
pub enum Error {
    VariableNotInitialized(String),
    InvaildNumberOfArguments,
    FailedToConvertType,
}
