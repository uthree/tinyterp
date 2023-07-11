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
        return Environment {
            store: BTreeMap::new(),
            outer: Some(Box::new(self)),
        };
    }

    // copy store
    pub fn copy_store(&self) -> Self {
        return Environment {
            store: self.store.clone(),
            outer: None,
        };
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

    // set objecct
    pub fn set(&mut self, name: String, value: Object) -> Object {
        self.store.insert(name, value.clone());
        return value;
    }

    pub fn evaluate(&mut self, node: &Node) -> Result<Object, Error> {
        if let Node::Sequence(seq, pos) = node {
            let mut last_obj = Object::Nil;
            for node in seq {
                match node {
                    Node::Return(n, p) => {
                        return self.evaluate_expression(n);
                    }
                    _ => {
                        let r = self.evaluate_expression(&node);
                        if r.is_ok() {
                            last_obj = r.unwrap();
                        } else {
                            return Err(r.err().unwrap());
                        }
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
            _ => Ok(Object::Nil),
        }
    }

    fn evaluate_sequence(&mut self, nodes: &Vec<Node>, pos: Position) -> Result<Object, Error> {
        let mut last_obj = Object::Nil;
        let mut env = self.copy_store().new_outer();
        for node in nodes {
            match node {
                Node::Return(n, p) => {
                    return env.evaluate_expression(n);
                }
                _ => {
                    let r = env.evaluate_expression(&node);
                    if r.is_ok() {
                        last_obj = r.unwrap();
                    } else {
                        return Err(r.err().unwrap());
                    }
                }
            }
        }
        Ok(last_obj)
    }

    fn evaluate_integer_literal(&mut self, i: i64, pos: Position) -> Result<Object, Error> {
        Ok(Object::Int(i))
    }

    fn evaluate_assign(
        &mut self,
        names: &Vec<String>,
        nodes: &Vec<Node>,
        pos: Position,
    ) -> Result<Object, Error> {
        if names.len() == 1 {
            let r = self.evaluate_expression(&nodes[0]);
            if r.is_ok() {
                let r = r.unwrap();
                self.set(names[0].clone(), r.clone());
                Ok(r.clone())
            } else {
                Err(r.err().unwrap())
            }
        } else {
            unreachable!();
        }
    }
}
