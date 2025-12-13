#[derive(Debug)]
pub enum Tokens {
    LET,
    EQUALS,
    IDENT(String),
    NUMBER(i32),
    SEMICOLON,
    NEWLINE,
    EOF,
}
