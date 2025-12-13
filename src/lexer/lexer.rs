use crate::tokens::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::process;

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*$").unwrap();
}

pub struct Lexer {
    src: String,
    pub toks: Vec<Tokens>,
    index: usize,
    line: usize,
}

impl Lexer {
    pub fn new(str: String) -> Self {
        Self {
            src: str,
            toks: Vec::new(),
            index: 0,
            line: 1,
        }
    }

    pub fn lexe(&mut self) {
        while self.index < self.src.len() {
            let mut char_iter = self.src[self.index..].chars();
            let current_char = char_iter.next();

            let char = match current_char {
                Some(c) => c,
                None => {
                    break;
                }
            };

            if char == '\n' {
                self.toks.push(Tokens::NEWLINE);
                self.line += 1;
                self.index += char.len_utf8();
                continue;
            } else if char.is_whitespace() {
                self.index += char.len_utf8();
                continue;
            }

            match char {
                ';' => {
                    self.toks.push(Tokens::SEMICOLON);
                    self.index += char.len_utf8();
                }
                '=' => {
                    self.toks.push(Tokens::EQUALS);
                    self.index += char.len_utf8();
                }
                '+' => {
                    self.toks.push(Tokens::PLUS);
                    self.index += char.len_utf8();
                }
                '-' => {
                    self.toks.push(Tokens::MINUS);
                    self.index += char.len_utf8();
                }
                _ if char.is_alphabetic() => {
                    let mut current_pos = self.index;
                    let mut id_iter = self.src[current_pos..].chars();
                    let mut identifier = String::new();

                    while let Some(c) = id_iter.next() {
                        if c.is_alphanumeric() || c == '_' {
                            identifier.push(c);
                            current_pos += c.len_utf8();
                        } else {
                            break;
                        }
                    }

                    match identifier.as_str() {
                        "let" => self.toks.push(Tokens::LET),
                        _ => self.toks.push(Tokens::IDENT(identifier)),
                    }

                    self.index = current_pos;
                }
                _ if char.is_numeric() => {
                    let mut current_pos = self.index;
                    let mut id_iter = self.src[current_pos..].chars();
                    let mut number = String::new();

                    while let Some(c) = id_iter.next() {
                        if c.is_numeric() || c == '_' {
                            number.push(c);
                            current_pos += c.len_utf8();
                        } else {
                            break;
                        }
                    }

                    self.toks
                        .push(Tokens::NUMBER(number.parse::<i32>().unwrap()));
                    self.index = current_pos;
                }
                _ => {
                    eprintln!(
                        "Lexer Error: Unknown character '{}' at line {}",
                        char, self.line
                    );
                    process::exit(1);
                }
            }
        }
        self.toks.push(Tokens::EOF);
    }
}
