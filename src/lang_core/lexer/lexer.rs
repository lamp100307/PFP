use crate::lang_core::error::{PFPError, Span};
use super::token::{TokT, Token};
use crate::string;

pub fn lex(input: String, file_path: String) -> Result<Vec<Token>, PFPError> {
    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();

    let mut i = 0;
    let mut line: u32 = 0;
    let mut line_start = 0;

    while i < len {
        let ch = chars[i];
        match ch {
            ' ' | '\t' | '\r' => {
                i += 1;
            }
            '\n' => {
                i += 1;
                line += 1;
                line_start = i;
            }
            '(' => {
                tokens.push(Token {
                    kind: TokT::LPAREN,
                    val: ch.to_string(),
                    span: Span::new(file_path.clone(), line, i as u32, i as u32, ch.to_string()),
                });
                i += 1;
            }
            ')' => {
                tokens.push(Token {
                    kind: TokT::RPAREN,
                    val: ch.to_string(),
                    span: Span::new(file_path.clone(), line, i as u32, i as u32, ch.to_string()),
                });
                i += 1;
            }
            '"' => {
                i += 1;
                let start = i;
                let mut end = i;
                while end < len && chars[end] != '"' {
                    end += 1;
                }
                let text: String = chars[start..end].iter().collect();
                tokens.push(Token {
                    kind: TokT::STRING,
                    val: text.clone(),
                    span: Span::new(
                        file_path.clone(),
                        line,
                        start as u32,
                        end as u32,
                        text,
                    ),
                });
                i = end + 1;
            }
            '+' | '-' | '*' | '/' => {
                tokens.push(Token {
                    kind: TokT::OP,
                    val: ch.to_string(),
                    span: Span::new(file_path.clone(), line, i as u32, i as u32, ch.to_string()),
                });
                i += 1;
            }
            '0'..='9' => {
                let start = i;
                let mut end = i;
                while end < len && chars[end].is_ascii_digit() {
                    end += 1;
                }

                let text: String = chars[start..end].iter().collect();

                if chars[end] == '.' {
                    end += 1;
                    while end < len && chars[end].is_ascii_digit() {
                        end += 1;
                    }
                    let text: String = chars[start..end].iter().collect();
                    tokens.push(Token {
                        kind: TokT::FLOAT,
                        val: text.clone(),
                        span: Span::new(
                            file_path.clone(),
                            line,
                            start as u32,
                            end as u32,
                            text,
                        ),
                    });
                    i = end;
                    continue;
                }

                tokens.push(Token {
                    kind: TokT::INT,
                    val: text.clone(),
                    span: Span::new(
                        file_path.clone(),
                        line,
                        start as u32,
                        end as u32,
                        text,
                    ),
                });

                i = end;
            }
            'a'..='z' | 'A'..='Z' => {
                let start = i;
                let mut end = i;
                while end < len && chars[end].is_ascii_alphanumeric() {
                    end += 1;
                }

                let text: String = chars[start..end].iter().collect();

                tokens.push(Token {
                    kind: TokT::IDENT,
                    val: text.clone(),
                    span: Span::new(
                        file_path.clone(),
                        line,
                        start as u32,
                        end as u32,
                        text,
                    ),
                });

                i = end;
            }
            _ => {
                let text: String = chars[line_start..=i].iter().collect();
                return Err(PFPError::new(
                    string!(string!("LEX")),
                    0001,
                    Span::new(
                        file_path,
                        line,
                        i as u32,
                        i as u32,
                        text,
                    ),
                    "unknown char".to_string(),
                ));
            }
        }
    }
    Ok(tokens)
}