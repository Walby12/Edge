mod lexer;
mod tokens;
use crate::lexer::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new(String::from("let x = 12;"));
    lexer.lexe();

    for t in lexer.toks {
        println!("TOKEN: {:?}", t);
    }
}
