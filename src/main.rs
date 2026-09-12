mod lang_core;

use lang_core::lexer::lexer::lex;
use lang_core::parser::parser;
use lang_core::interpreter::interpreter::Interpreter;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let file_path = &args[1];
    let input = std::fs::read_to_string(file_path).expect("File not found");

    let mut tokens_ = Vec::new();
    match lex(input, file_path.to_string()) {
        Ok(tokens) => {
            tokens_ = tokens.clone();
            println!("Tokens:\n{}\n\n\n", tokens.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"))
        },
        Err(err) => err.print(),
    }

    let mut parser_ = parser::Parser { tokens: tokens_, pos: 0, last: None, file_path: file_path.to_string() };

    let mut ast_ = Vec::new();
    match parser_.parse() {
        Ok(ast) => {
            ast_ = ast.clone();
            println!("AST:\n{}", ast.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"))
        },
        Err(err) => err.print(),
    }

    let interpreter = Interpreter::new(ast_);
    interpreter.run();
}
