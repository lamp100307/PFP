use crate::lang_core::error::{PFPError, Span};
use crate::lang_core::lexer::token::{TokT, Token};
use crate::lang_core::parser::ast::NodeKind::Int;
use super::ast::{Node, NodeKind};

pub(crate) struct Parser {
    pub(crate) pos: u32,
    pub(crate) tokens: Vec<Token>,
    pub(crate) last: Option<Token>,
    pub(crate) file_path: String
}

impl Parser {
    fn eof(&self) -> PFPError {
        let span = self.last.as_ref()
            .map(|t| t.span.clone())
            .unwrap_or_else(|| Span::new(self.file_path.clone(), 0, 0, 0, String::new()));
        PFPError::new("Parser".to_string(), 0, span, "Unexpected EOF".to_string())
    }

    fn peek(&self) -> Option<&Token> {
        if self.pos < self.tokens.len() as u32 {
            Some(&self.tokens[self.pos as usize])
        } else {
            None
        }
    }

    fn consume(&mut self, type_: TokT) -> Option<Token> {
        if let Some(tok) = self.tokens.get(self.pos as usize) {
            if tok.kind == type_ {
                let tok = tok.clone();
                self.pos += 1;
                return Some(tok);
            }
        }
        None
    }

    fn peek_op(&self, ops: &[&str]) -> bool {
        matches!(self.peek(), Some(t) if t.kind == TokT::OP && ops.contains(&t.val.as_str()))
    }

    fn parse_expr(&mut self) -> Result<Node, PFPError> {
        let mut left = self.parse_mult()?;
        while self.peek_op(&["+", "-"]) {
            let tok = self.consume(TokT::OP).unwrap();
            let right = self.parse_mult()?;
            left = Node {
                kind: NodeKind::BOP(tok.val, Box::new(left.clone()), Box::new(right.clone())),
                span: Span::union(left.span.clone(), right.span.clone()),
            };
        }
        Ok(left)
    }

    fn parse_mult(&mut self) -> Result<Node, PFPError> {
        let mut left = self.parse_atom()?;
        while self.peek_op(&["*", "/"]) {
            let tok = self.consume(TokT::OP).unwrap();
            let right = self.parse_atom()?;
            left = Node {
                kind: NodeKind::BOP(tok.val, Box::new(left.clone()), Box::new(right.clone())),
                span: Span::union(left.span.clone(), right.span.clone()),
            };
        }
        Ok(left)
    }
    fn parse_atom(&mut self) -> Result<Node, PFPError> {
        let token = match self.peek() {
            Some(t) => t.clone(),
            None => return Err(self.eof()),
        };

        self.last = Some(token.clone());
        self.pos += 1;

        match token.kind {
            TokT::INT => {
                let val = token.val.parse::<i32>().map_err(|_| {
                    PFPError::new("Parser".to_string(), 0, token.span.clone(), "invalid int".to_string())
                })?;
                Ok(Node { kind: Int(val), span: token.span })
            }
            TokT::FLOAT => {
                let val = token.val.parse::<f64>().map_err(|_| {
                    PFPError::new("Parser".to_string(), 0, token.span.clone(), "invalid float".to_string())
                })?;
                Ok(Node { kind: NodeKind::Float(val), span: token.span })
            }
            TokT::STRING => {
                Ok(Node { kind: NodeKind::String(token.val), span: token.span })
            }
            TokT::IDENT => {
                if self.consume(TokT::LPAREN).is_some() {
                    let mut args: Vec<Node> = Vec::new();
                    while self.peek().map_or(false, |t| t.kind != TokT::RPAREN) {
                        args.push(self.parse_expr()?);
                        if self.consume(TokT::COMMA).is_none() {
                            break;
                        }
                    }
                    self.consume(TokT::RPAREN).unwrap();
                    Ok(Node { kind: NodeKind::FuncCall(token.val, args), span: token.span })
                } else {
                    Err(PFPError::new(
                        "Parser".to_string(),
                        0,
                        token.span,
                        "Variables not supported".to_string(),
                    ))
                }
            }
            _ => Err(PFPError::new(
                "Parser".to_string(),
                0,
                token.span.clone(),
                "Unexpected token".to_string(),
            )),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, PFPError> {
        let mut nodes: Vec<Node> = Vec::new();
        while self.peek().is_some() {
            match self.parse_expr() {
                Ok(node) => nodes.push(node),
                Err(err) => return Err(err),
            }
        }
        Ok(nodes)
    }
}