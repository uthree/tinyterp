#[derive(Debug)]
pub enum NodeType {
    IntegerLiteral,
    OperatorAdd,
    OpeartorSub,
    OperatorMul,
    OperatorDiv,
}

#[derive(Debug)]
pub struct Node {
    nodetype: NodeType,
    literal: String,
    childlen: Vec<Box<Node>>,
}

impl Node {
    pub fn new(nodetype: NodeType, literal: String, childlen: Vec<Box<Node>>) -> Node {
        return Node {
            nodetype,
            literal,
            childlen,
        };
    }
}
