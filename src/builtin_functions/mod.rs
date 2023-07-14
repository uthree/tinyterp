pub mod cast;
pub mod math;
pub mod print;

use crate::builtin_functions::cast::{get_type, to_float, to_int, to_str};
use crate::builtin_functions::math::{
    abs, acos, asin, atan, cos, cosh, exp, ln, modulo, sin, sinh, sqrt, tan, tanh,
};
use crate::builtin_functions::print::builtin_print;

use crate::core::environment::Environment;
use crate::core::object::Object;

pub fn load_builtin_functions(env: &mut Environment) {
    env.add_function("str", to_str);
    env.add_function("int", to_int);
    env.add_function("float", to_float);
    env.add_function("type", get_type);

    env.add_function("exp", exp);
    env.add_function("sin", sin);
    env.add_function("cos", cos);
    env.add_function("tan", tan);
    env.add_function("sinh", sinh);
    env.add_function("cosh", cosh);
    env.add_function("tanh", tanh);
    env.add_function("asin", asin);
    env.add_function("acos", acos);
    env.add_function("atan", atan);
    env.add_function("ln", ln);
    env.add_function("sqrt", sqrt);
    env.add_function("abs", abs);
    env.add_function("mod", modulo);

    //set pi
    env.set("pi".to_string(), Object::Float(std::f64::consts::PI));
}

pub fn load_builtin_print(env: &mut Environment) {
    env.add_function("print", builtin_print)
}
