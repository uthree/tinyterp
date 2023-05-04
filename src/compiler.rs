use crate::object::Object;
use crate::parser::Node;
use crate::runtime::Instruction;

pub fn compile(node: Node) -> Vec<Instruction> {
    match node {
        Node::IntegerLiteral(i) => vec![Instruction::Push(Object::Int(i.parse::<i64>().unwrap()))],
        Node::Add(v1, v2) => {
            let mut v1 = compile(*v1);
            let mut v2 = compile(*v2);
            let mut out = vec![];
            out.append(&mut v1);
            out.append(&mut v2);
            out.push(Instruction::Add);
            return out;
        }
        Node::Sub(v1, v2) => {
            let mut v1 = compile(*v1);
            let mut v2 = compile(*v2);
            let mut out = vec![];
            out.append(&mut v1);
            out.append(&mut v2);
            out.push(Instruction::Sub);
            return out;
        }
        _ => {
            vec![]
        }
    }
}
