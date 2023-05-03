use std::collections::HashMap;

use crate::object::Object;

#[derive(Clone, Debug)]
pub enum Instruction {
    Push(Object),
    Add,
}

pub struct Runtime {
    stack: Vec<Object>,
    heap: Object,
    instructions: Vec<Instruction>,
    program_counter: usize,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            stack: vec![],
            heap: Object::Namespace(HashMap::new()),
            instructions: vec![],
            program_counter: 0,
        }
    }

    pub fn push_instructions(&mut self, mut insts: Vec<Instruction>) {
        self.instructions.append(&mut insts);
    }

    pub fn run(&mut self) -> Result<(), &str> {
        while self.program_counter < self.instructions.len() {
            match &self.instructions[self.program_counter] {
                Instruction::Push(obj) => {
                    self.stack.push(obj.clone());
                }
                Instruction::Add => {
                    let v1 = self.stack.pop().expect("Stack underflow");
                    let v2 = self.stack.pop().expect("Stack underflow");
                    self.stack.push(v1.op_add(&v2).expect("Failed to add"));
                }
            }
            self.program_counter += 1;
        }
        Ok(())
    }
}
