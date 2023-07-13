pub mod cast;
pub mod print;

use crate::builtin_functions::cast::{get_type, to_str};
use crate::builtin_functions::print::builtin_print;

use crate::core::environment::Environment;

pub fn load_builtin_functions(env: &mut Environment) {
    env.add_function("str", to_str);
    env.add_function("type", get_type);
}

pub fn load_builtin_print(env: &mut Environment) {
    env.add_function("print", builtin_print)
}
