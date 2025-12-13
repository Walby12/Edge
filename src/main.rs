mod codegen;
mod lexer;
mod parser;
mod symbol_table;
mod tokens;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

fn main() {
    let mut lexer = Lexer::new(String::from("let x = 12;"));
    lexer.lexe();

    let mut parser = Parser::new(lexer.toks);
    parser.parse();
}
