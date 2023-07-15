use crate::core::parser::Position;

#[derive(Debug)]
pub enum Error {
    VariableNotInitialized(String, Position),
    TypeError(String, Position),
    ArgumentError(String, Position),
    DivideByZero(String, Position),
    IndexOutOfRange(String, Position),
    ParseError(String, Position),
}

pub fn generate_error_message(error: Error, code: &str) -> String {
    let mut output = "".to_string();
    let _code = code.to_string();

    output = format!("{:?}", error);
    output
}
