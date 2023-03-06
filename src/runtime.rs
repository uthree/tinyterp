use crate::compiler::compile;
use crate::grammar::tinyterp::program;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Operator {
    InitializeInteger,
    InitializeFloat,
    InitializeObject,
    Jump,
    Return,
    Exit,
}

#[derive(Debug)]
pub struct Instruction {
    operator: Operator,
    argument1: usize,
    argument2: usize,
    argument3: usize,
}

impl Instruction {
    pub fn new(op: Operator, a1: usize, a2: usize, a3: usize) -> Self {
        return Instruction {
            operator: op,
            argument1: a1,
            argument2: a2,
            argument3: a3,
        };
    }
}

enum PrimitiveObject {
    Integer(i64),
}

enum TinyterpObject {}

enum Object {
    TinyterpObject,
    PrimitiveObject,
}

#[derive(Default)]
struct Scope {
    Variables: HashMap<String, usize>,
}

#[derive(Default)]
pub struct Runtime {
    heap: HashMap<usize, Object>,
    call_stack: Vec<usize>,
    program_pointer: usize,
    RootScope: Scope,
}

impl Runtime {
    pub fn new() -> Runtime {
        let mut r = Runtime::default();
        r.program_pointer = 0;
        return r;
    }

    fn execute_instructions(mut self, inst: &Vec<Instruction>) {
        loop {
            let i = &inst[self.program_pointer];
            match i.operator {
                Operator::Exit => break,
                _ => {}
            }
            self.program_pointer += 1;
        }
    }

    pub fn execute_code(self, code: &str) {
        self.execute_instructions(&compile(program(code).unwrap()));
    }
}
