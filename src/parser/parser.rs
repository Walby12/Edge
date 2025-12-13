use crate::codegen::codegen::Codegen;
use crate::symbol_table::{SymbolTable, VariableType};
use crate::tokens::Tokens;
use std::mem;
use std::process;

pub struct Parser {
    symbol_table: SymbolTable,
    codegen: Codegen,
    toks: Vec<Tokens>,
    index: usize,
    line: usize,
}

fn is_same_variant(a: &Tokens, b: &Tokens) -> bool {
    mem::discriminant(a) == mem::discriminant(b)
}

impl Parser {
    pub fn new(t: Vec<Tokens>) -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            codegen: Codegen::new("test.c".to_string()),
            toks: t,
            index: 0,
            line: 1,
        }
    }

    fn current(&self) -> &Tokens {
        self.toks.get(self.index).unwrap_or(&Tokens::EOF)
    }

    fn expect(&mut self, expected_tok: &Tokens) -> Tokens {
        let cur_tok = self.current();

        let matches = if expected_tok == cur_tok {
            true
        } else if is_same_variant(expected_tok, cur_tok) {
            true
        } else {
            false
        };

        if !matches {
            eprintln!(
                "ERROR on line {}: Expected {:?} but got: {:?}",
                self.line, expected_tok, cur_tok
            );
            process::exit(1);
        }

        let consumed_token = cur_tok.clone();
        self.index += 1;

        consumed_token
    }

    fn consume_ident_value(&mut self) -> String {
        let token = self.expect(&Tokens::IDENT("".to_string()));
        match token {
            Tokens::IDENT(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn parse(&mut self) {
        while *self.current() != Tokens::EOF {
            match self.current() {
                Tokens::LET => self.parse_let_stmt(),
                Tokens::NEWLINE => {
                    self.line += 1;
                    self.index += 1;
                }
                _ => todo!(),
            }
        }
    }

    fn parse_let_stmt(&mut self) {
        self.expect(&Tokens::LET);
        let var_name = self.consume_ident_value();
        self.expect(&Tokens::EQUALS);

        let var_type: VariableType;
        match self.current() {
            Tokens::NUMBER(n) => var_type = VariableType::INT32(*n),
            _ => {
                eprintln!(
                    "ERROR on line {}: Expected {:?} but got: {:?}",
                    self.line,
                    "NUMBER",
                    self.current()
                );
                process::exit(1);
            }
        }
        self.index += 1;
        self.expect(&Tokens::SEMICOLON);

        self.codegen.let_stmt(&var_name, &var_type);
        self.symbol_table.set(var_name, var_type);
    }
}
