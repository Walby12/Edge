#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    LET,
    EQUALS,
    IDENT(String),
    NUMBER(i32),
    SEMICOLON,
    NEWLINE,
    PLUS,
    MINUS,
    EOF,
}
