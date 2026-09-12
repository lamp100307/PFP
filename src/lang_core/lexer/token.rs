// Token definitions

use crate::lang_core::error::Span;

#[derive(PartialEq, Clone)]
pub enum TokT {
    INT, FLOAT,
    STRING,
    IDENT,
    LPAREN, RPAREN,
    COMMA,
    OP
}

impl std::fmt::Display for TokT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokT::INT => write!(f, "INT"),
            TokT::FLOAT => write!(f, "FLOAT"),
            TokT::STRING => write!(f, "STRING"),
            TokT::IDENT => write!(f, "IDENT"),
            TokT::LPAREN => write!(f, "LPAREN"),
            TokT::RPAREN => write!(f, "RPAREN"),
            TokT::COMMA => write!(f, "COMMA"),
            TokT::OP => write!(f, "OP"),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub kind: TokT,
    pub val: String,
    pub span: Span
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.val)
    }
}