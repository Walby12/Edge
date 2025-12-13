use crate::tokens::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::process;

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*$").unwrap();
}

// Removed use of std::process; - handle errors gracefully instead

pub fn lexe(src: &str, toks: &mut Vec<Tokens>, index: &mut usize, line: &mut usize) {
    while *index < src.len() {
        let mut char_iter = src[*index..].chars();
        let current_char = char_iter.next();

        let char = match current_char {
            Some(c) => c,
            None => {
                break;
            }
        };

        if char == '\n' {
            toks.push(Tokens::NEWLINE);
            *line += 1;
            *index += char.len_utf8();
            continue;
        } else if char.is_whitespace() {
            *index += char.len_utf8();
            continue;
        }

        match char {
            ';' => {
                toks.push(Tokens::SEMICOLON);
                *index += char.len_utf8();
            }
            '=' => {
                toks.push(Tokens::EQUALS);
                *index += char.len_utf8();
            }
            _ if char.is_alphabetic() => {
                let mut current_pos = *index;
                let mut id_iter = src[current_pos..].chars();
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
                    "let" => toks.push(Tokens::LET),
                    _ => toks.push(Tokens::IDENT(identifier)),
                }

                *index = current_pos;
            }
            _ if char.is_numeric() => {
                let mut current_pos = *index;
                let mut id_iter = src[current_pos..].chars();
                let mut number = String::new();

                while let Some(c) = id_iter.next() {
                    if c.is_numeric() || c == '_' {
                        number.push(c);
                        current_pos += c.len_utf8();
                    } else {
                        break;
                    }
                }

                toks.push(Tokens::NUMBER(number.parse::<i32>().unwrap()));
                *index = current_pos;
            }
            _ => {
                eprintln!(
                    "Lexer Error: Unknown character '{}' at line {}",
                    char, *line
                );
                process::exit(1);
            }
        }
    }
    toks.push(Tokens::EOF);
}
