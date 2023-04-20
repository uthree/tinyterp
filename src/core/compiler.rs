// compile AST to VM Instructs
use crate::core::parser::Node;
use crate::core::runtime::{Instruction, Variable};

pub fn compile(node: Node) -> Vec<Instruction> {
    let mut insts = vec![];
    match node {
        Node::IntegerLiteral(v) => {
            insts.push(Instruction::Push(Variable::Integer(
                v.parse::<isize>().unwrap(),
            )));
        }
        Node::OperatorAdd(v1, v2) => {
            insts.append(&mut compile(*v1));
            insts.append(&mut compile(*v2));
            insts.push(Instruction::Add);
        }

        _ => {}
    }
    insts
}
