pub mod cast;

use crate::builtin_functions::cast::{get_type, to_str};

use crate::core::environment::Environment;

pub fn load_builtin_functions(env: &mut Environment) {
    env.add_function("str", to_str);
    env.add_function("type", get_type);
}
