mod lexer;
mod tokens;

fn main() {
    let mut v: Vec<tokens::Tokens> = Vec::new();
    lexer::lexer::lexe("let x_12 = 12;", &mut v, &mut 0, &mut 1);

    for t in v {
        println!("TOKEN: {:?}", t);
    }
}
