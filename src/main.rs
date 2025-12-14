mod codegen;
mod lexer;
mod parser;
mod symbol_table;
mod tokens;
use crate::parser::parser::Parser;

fn main() {
    let mut parser = Parser::new("main :: void {let x = 12;}".to_string());
    parser.parse();
}
