use crate::builtin_functions::load_builtin_functions;
use crate::core::error::Error;
use crate::core::object::Object;
use crate::core::parser::Node;
use crate::core::parser::Position;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct Environment {
    store: BTreeMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            store: BTreeMap::new(),
            outer: None,
        };
        load_builtin_functions(&mut env);
        env
    }

    // initialize new scope
    pub fn new_outer(self) -> Self {
        Environment {
            store: BTreeMap::new(),
            outer: Some(Box::new(self)),
        }
    }

    // clone store
    pub fn clone_store(&self) -> Self {
        Environment {
            store: self.store.clone(),
            outer: None,
        }
    }

    // get object
    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(value) => Some(value.clone()),
            None => match &self.outer {
                Some(outer) => outer.get(name),
                None => None,
            },
        }
    }

    // set object
    pub fn set(&mut self, name: String, value: Object) -> Object {
        self.store.insert(name, value.clone());
        value
    }

    // drop object
    pub fn drop(&mut self, name: String) {
        self.store.remove(&name);
    }

    // add built-in(Rust) function
    pub fn add_function(
        &mut self,
        name: &str,
        function: fn(Vec<Object>, BTreeMap<String, Object>, Position) -> Result<Object, Error>,
    ) {
        self.store
            .insert(name.to_string(), Object::BuiltInFunction(function));
    }

    pub fn evaluate_program(&mut self, node: &Node) -> Result<Object, Error> {
        if let Node::Sequence(seq, _pos) = node {
            let mut last_obj = Object::Nil;
            for node in seq {
                match node {
                    Node::Return(n, _p) => {
                        return self.evaluate_expression(n);
                    }
                    _ => {
                        last_obj = self.evaluate_expression(node)?;
                    }
                }
            }
            Ok(last_obj)
        } else {
            unreachable!();
        }
    }

    fn evaluate_expression(&mut self, node: &Node) -> Result<Object, Error> {
        match node {
            Node::Sequence(seq, pos) => self.evaluate_sequence(seq, *pos),
            Node::IntegerLiteral(i, pos) => self.evaluate_integer_literal(*i, *pos),
            Node::FloatLiteral(f, pos) => self.evaluate_float_literal(*f, *pos),
            Node::StringLiteral(s, pos) => self.evaluate_str_literal(s.clone(), *pos),
            Node::Assign(names, nodes, pos) => self.evaluate_assign(names, nodes, *pos),
            Node::Identifier(name, pos) => self.evaluate_identifier(name, *pos),
            Node::Function {
                arguments,
                keyword_arguments,
                sequence,
                position,
            } => self.evaluate_function(
                &arguments.clone(),
                keyword_arguments.clone(),
                *sequence.clone(),
                *position,
            ),
            Node::CallFunction {
                callable,
                arguments,
                keyword_arguments,
                position,
            } => self.evaluate_call_function(
                callable,
                arguments,
                keyword_arguments.clone(),
                *position,
            ),
            Node::Drop(names, pos) => self.evaluate_drop(names, *pos),
            Node::List(nodes, pos) => self.evaluate_list(nodes, *pos),
            Node::Add(left, right, pos) => self.evaluate_add(left, right, *pos),
            Node::Sub(left, right, pos) => self.evaluate_sub(left, right, *pos),
            Node::Div(left, right, pos) => self.evaluate_div(left, right, *pos),
            Node::Mul(left, right, pos) => self.evaluate_mul(left, right, *pos),
            Node::Pow(left, right, pos) => self.evaluate_pow(left, right, *pos),
            Node::Neg(value, pos) => self.evaluate_neg(value, *pos),
            _ => Ok(Object::Nil),
        }
    }

    fn evaluate_neg(&mut self, value: &Node, pos: Position) -> Result<Object, Error> {
        self.evaluate_expression(value)?.neg(pos)
    }

    fn evaluate_add(&mut self, left: &Node, right: &Node, pos: Position) -> Result<Object, Error> {
        self.evaluate_expression(left)?
            .add(self.evaluate_expression(right)?, pos)
    }

    fn evaluate_sub(&mut self, left: &Node, right: &Node, pos: Position) -> Result<Object, Error> {
        self.evaluate_expression(left)?
            .sub(self.evaluate_expression(right)?, pos)
    }

    fn evaluate_mul(&mut self, left: &Node, right: &Node, pos: Position) -> Result<Object, Error> {
        self.evaluate_expression(left)?
            .mul(self.evaluate_expression(right)?, pos)
    }

    fn evaluate_div(&mut self, left: &Node, right: &Node, pos: Position) -> Result<Object, Error> {
        self.evaluate_expression(left)?
            .div(self.evaluate_expression(right)?, pos)
    }

    fn evaluate_pow(&mut self, left: &Node, right: &Node, pos: Position) -> Result<Object, Error> {
        self.evaluate_expression(left)?
            .pow(self.evaluate_expression(right)?, pos)
    }

    fn evaluate_list(&mut self, nodes: &[Node], _pos: Position) -> Result<Object, Error> {
        let mut elements = vec![];
        for node in nodes {
            elements.push(self.evaluate_expression(node)?)
        }
        Ok(Object::List(elements))
    }

    fn evaluate_call_function(
        &mut self,
        callable: &Node,
        arg_nodes: &[Node],
        kwarg_nodes: HashMap<String, Node>,
        pos_call: Position,
    ) -> Result<Object, Error> {
        let callable_obj = self.evaluate_expression(callable)?;
        match callable_obj {
            Object::BuiltInFunction(func) => {
                let mut args_vec = vec![];
                let mut kwargs_hash = BTreeMap::new();
                for arg in arg_nodes {
                    args_vec.push(self.evaluate_expression(arg)?);
                }
                for (key, value) in kwarg_nodes.iter() {
                    kwargs_hash.insert(key.clone(), self.evaluate_expression(value)?);
                }
                func(args_vec, kwargs_hash, pos_call)
            }
            Object::Function {
                args,
                kwargs,
                body,
                mut env,
                pos: _,
            } => {
                // check number of arguments
                if arg_nodes.len() != args.len() {
                    return Err(Error::ArgumentError(
                        format!(
                            "the function takes {} positional arguments, but {} positional arguments were given.",
                            args.len(),
                            arg_nodes.len()
                        )
                        ,
                        pos_call,
                    ));
                }
                //TODO: detect invalid keyword_arguments ex: f = () -> {}; f(a=1)

                // set arguments
                for (key, value_node) in args.iter().zip(arg_nodes.iter()) {
                    env.set(key.to_string(), self.evaluate_expression(value_node)?);
                }
                for key in kwargs.keys() {
                    let mut value_node = None;
                    if let Some(value) = kwarg_nodes.get(key) {
                        value_node = Some(value)
                    } else {
                        value_node = Some(kwargs.get(key).unwrap())
                    }
                    let r = self.evaluate_expression(value_node.unwrap())?;
                    env.set(key.to_string(), r);
                }
                // call function
                env.evaluate_expression(&body)
            }
            _ => {
                let c_obj = self.evaluate_expression(callable)?;
                Err(Error::TypeError(
                    format!("{} is not callable.", c_obj.type_name()),
                    pos_call,
                ))
            }
        }
    }

    fn evaluate_function(
        &mut self,
        args: &[String],
        kwargs: HashMap<String, Node>,
        sequence: Node,
        pos: Position,
    ) -> Result<Object, Error> {
        Ok(Object::Function {
            args: args.to_vec(),
            kwargs,
            body: sequence,
            pos,
            env: self.clone_store(),
        })
    }

    fn evaluate_sequence(&mut self, nodes: &[Node], _pos: Position) -> Result<Object, Error> {
        let mut last_obj = Object::Nil;
        let mut env = self.clone_store().new_outer();
        for node in nodes {
            match node {
                Node::Return(n, _p) => {
                    return env.evaluate_expression(n);
                }
                _ => {
                    last_obj = env.evaluate_expression(node)?;
                }
            }
        }
        Ok(last_obj)
    }

    fn evaluate_integer_literal(&mut self, i: i64, _pos: Position) -> Result<Object, Error> {
        Ok(Object::Int(i))
    }

    fn evaluate_float_literal(&mut self, f: f64, _pos: Position) -> Result<Object, Error> {
        Ok(Object::Float(f))
    }

    fn evaluate_str_literal(&mut self, s: String, _pos: Position) -> Result<Object, Error> {
        Ok(Object::Str(s))
    }

    fn evaluate_assign(
        &mut self,
        lefts: &[Node],
        rights: &[Node],
        pos: Position,
    ) -> Result<Object, Error> {
        if lefts.len() == 1 {
            let r = self.evaluate_expression(&rights[0])?;
            match &lefts[0] {
                Node::Identifier(name, _n_pos) => {
                    self.set(name.clone(), r.clone());
                }
                _ => {
                    todo!()
                }
            }
            Ok(r)
        } else {
            let mut output = vec![];
            for (l, r) in lefts.iter().zip(rights.iter()) {
                output.push(self.evaluate_assign(&[l.clone()], &[r.clone()], pos)?)
            }
            Ok(Object::List(output))
        }
    }

    fn evaluate_identifier(&mut self, name: &String, pos: Position) -> Result<Object, Error> {
        self.get(name)
            .ok_or(Error::VariableNotInitialized(name.clone(), pos))
    }

    fn evaluate_drop(&mut self, names: &[String], pos: Position) -> Result<Object, Error> {
        for name in names {
            if self.store.contains_key(name) {
                self.drop(name.clone());
            } else {
                return Err(Error::VariableNotInitialized(name.clone(), pos));
            }
        }
        Ok(Object::Nil)
    }
}
