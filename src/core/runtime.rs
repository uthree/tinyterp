use std::any::Any;

use crate::core::compiler::compile;
use crate::core::parser::tinyterp::program as parse;

#[derive(Debug, Clone)]
pub enum Variable {
    Integer(isize),
}

impl std::ops::Add for Variable {
    type Output = Variable;
    fn add(self, other: Variable) -> Variable {
        match self {
            Variable::Integer(s) => match other {
                Variable::Integer(o) => Variable::Integer(s + o),
                _ => {
                    panic!("TypeError");
                }
            },
        }
    }
}

impl std::ops::Sub for Variable {
    type Output = Variable;
    fn sub(self, other: Variable) -> Variable {
        match self {
            Variable::Integer(s) => match other {
                Variable::Integer(o) => Variable::Integer(s - o),
                _ => {
                    panic!("TypeError");
                }
            },
        }
    }
}

impl std::ops::Mul for Variable {
    type Output = Variable;
    fn mul(self, other: Variable) -> Variable {
        match self {
            Variable::Integer(s) => match other {
                Variable::Integer(o) => Variable::Integer(s * o),
                _ => {
                    panic!("TypeError");
                }
            },
        }
    }
}

impl std::ops::Div for Variable {
    type Output = Variable;
    fn div(self, other: Variable) -> Variable {
        match self {
            Variable::Integer(s) => match other {
                Variable::Integer(o) => Variable::Integer(s / o),
                _ => {
                    panic!("TypeError");
                }
            },
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Push(Variable),
    Assign(String, Variable),
    Add,
    Sub,
    Mul,
    Div,
    ClearStack,
    Exit,
}

pub struct Runtime {
    stack: Vec<Variable>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime { stack: Vec::new() }
    }
    fn execute_insts(&mut self, insts: Vec<Instruction>) {
        let mut program_counter = 0;
        while program_counter < insts.len() {
            let inst = &insts[program_counter];
            program_counter += 1;
            match inst {
                Instruction::Push(v) => {
                    self.stack.push(v.clone());
                }
                Instruction::Add => {
                    let v1 = self.stack.pop().unwrap();
                    let v2 = self.stack.pop().unwrap();
                    let v_out = v2 + v1;
                    self.stack.push(v_out);
                }
                Instruction::Sub => {
                    let v1 = self.stack.pop().unwrap();
                    let v2 = self.stack.pop().unwrap();
                    let v_out = v2 - v1;
                    self.stack.push(v_out);
                }
                Instruction::Mul => {
                    let v1 = self.stack.pop().unwrap();
                    let v2 = self.stack.pop().unwrap();
                    let v_out = v2 * v1;
                    self.stack.push(v_out);
                }
                Instruction::Div => {
                    let v1 = self.stack.pop().unwrap();
                    let v2 = self.stack.pop().unwrap();
                    let v_out = v2 / v1;
                    self.stack.push(v_out);
                }
                _ => {}
            }
        }
    }

    pub fn execute(&mut self, input: &str) -> Variable {
        let node = parse(input).unwrap();
        self.execute_insts(compile(node));
        self.stack.pop().unwrap()
    }
}
