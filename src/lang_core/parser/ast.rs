use crate::lang_core::error::Span;

#[derive(Debug, Clone)]
pub enum NodeKind {
    Int(i32),
    Float(f64),
    String(String),
    FuncCall(String, Vec<Node>),
    BOP(String, Box<Node>, Box<Node>),
}

impl NodeKind {
    pub fn to_string(&self) -> String {
        match self {
            NodeKind::Int(val) => val.to_string(),
            NodeKind::Float(val) => val.to_string(),
            NodeKind::String(val) => format!("\"{}\"", val),
            NodeKind::FuncCall(name, args) => format!("{}({})", name, args.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", ")),
            NodeKind::BOP(op, left, right) => format!("({} {} {})", left.to_string(), op, right.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span
}

impl Node {
    pub fn to_string(&self) -> String {
        self.kind.to_string()
    }
}