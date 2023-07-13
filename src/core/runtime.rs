use crate::core::environment::Environment;
use crate::core::error::{generate_error_message, Error};
use crate::core::object::Object;
use crate::core::parser::tinyterp::program as parse;
pub use crate::core::parser::Position;
use std::collections::BTreeMap;

pub struct Runtime {
    pub env: Environment,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            env: Environment::new(),
        }
    }
    pub fn evaluate(&mut self, code: &str) -> Result<Object, String> {
        let node = parse(code);
        if let Ok(node) = node {
            let output = self.env.evaluate_program(&node);
            if let Ok(output) = output {
                Ok(output.remove_return())
            } else {
                Err(generate_error_message(output.err().unwrap(), code))
            }
        } else {
            Err("Syntax Error".to_string())
        }
    }

    // add built-in(Rust) function
    pub fn add_function(
        &mut self,
        name: &str,
        function: fn(Vec<Object>, BTreeMap<String, Object>, Position) -> Result<Object, Error>,
    ) {
        self.env.add_function(name, function);
    }
}
