// Compile AST to VM-Bytecodes

use crate::ast::Node;
use crate::runtime::Instruction;
use crate::runtime::Operator;

pub fn compile(root: Node) -> Vec<Instruction> {
    let mut inst = vec![];
    &inst.push(Instruction::new(Operator::Exit, 0, 0, 0));
    return inst;
}
