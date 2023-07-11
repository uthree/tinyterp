use crate::core::error::Error;
use crate::core::object::Object;
use crate::core::parser::Node;
use crate::core::parser::Position;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Environment {
    store: BTreeMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: BTreeMap::new(),
            outer: None,
        }
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
            Node::Assign(names, nodes, pos) => self.evaluate_assign(names, nodes, *pos),
            Node::Identifier(name, pos) => self.evaluate_identifier(name, *pos),
            _ => Ok(Object::Nil),
        }
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

    fn evaluate_assign(
        &mut self,
        names: &[String],
        nodes: &[Node],
        _pos: Position,
    ) -> Result<Object, Error> {
        if names.len() == 1 {
            let r = self.evaluate_expression(&nodes[0])?;
            self.set(names[0].clone(), r.clone());
            Ok(r)
        } else {
            todo!();
        }
    }

    fn evaluate_identifier(&mut self, name: &String, pos: Position) -> Result<Object, Error> {
        self.get(name)
            .ok_or(Error::VariableNotInitialized(name.clone()))
    }
}
