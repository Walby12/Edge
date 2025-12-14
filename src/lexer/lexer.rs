use crate::tokens::Tokens;
use std::process;

pub struct Lexer {
    src: String,
    index: usize,
    pub line: usize,
}

impl Lexer {
    pub fn new(str: String) -> Self {
        Self {
            src: str,
            index: 0,
            line: 1,
        }
    }

    pub fn next_token(&mut self) -> Tokens {
        loop {
            if self.index >= self.src.len() {
                return Tokens::EOF;
            }

            let current_char = self.src[self.index..].chars().next().unwrap();
            let char_len = current_char.len_utf8();

            if current_char == '\n' {
                self.line += 1;
            } else if !current_char.is_whitespace() {
                break;
            }

            self.index += char_len;
        }

        let mut char = self.src[self.index..].chars().next().unwrap();
        let mut char_len = char.len_utf8();

        match char {
            ';' => {
                self.index += char_len;
                Tokens::SEMICOLON
            }
            '=' => {
                self.index += char_len;
                Tokens::EQUALS
            }
            '+' => {
                self.index += char_len;
                Tokens::PLUS
            }
            '-' => {
                self.index += char_len;
                Tokens::MINUS
            }
            ':' => {
                char_len = char.len_utf8();
                self.index += char_len;
                char = self.src[self.index..].chars().next().unwrap();

                if char == ':' {
                    self.index += char_len;
                    Tokens::DOUBLECOL
                } else {
                    eprintln!("ERROR on line {}: Unknown char: ':'", self.line);
                    process::exit(1);
                }
            }
            '{' => {
                self.index += char_len;
                Tokens::OPENCURLY
            }
            '}' => {
                self.index += char_len;
                Tokens::CLOSECURLY
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

                self.index = current_pos;

                match identifier.as_str() {
                    "let" => Tokens::LET,
                    _ => Tokens::IDENT(identifier),
                }
            }

            _ if char.is_numeric() => {
                let mut current_pos = self.index;
                let mut num_iter = self.src[current_pos..].chars();
                let mut number_str = String::new();

                while let Some(c) = num_iter.next() {
                    if c.is_numeric() {
                        number_str.push(c);
                        current_pos += c.len_utf8();
                    } else {
                        break;
                    }
                }

                self.index = current_pos;
                match number_str.parse::<i32>() {
                    Ok(n) => Tokens::NUMBER(n),
                    Err(_) => {
                        eprintln!(
                            "ERROR on line {}: Invalid number format or overflow '{}'",
                            self.line, number_str
                        );
                        process::exit(1);
                    }
                }
            }
            _ => {
                eprintln!("ERROR on line {}: Unknown character '{}'", self.line, char);
                process::exit(1);
            }
        }
    }
}
