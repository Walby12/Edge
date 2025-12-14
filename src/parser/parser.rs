use crate::codegen::codegen::Codegen;
use crate::lexer::lexer::Lexer;
use crate::symbol_table::{FunctionType, SymbolTable, VariableType};
use crate::tokens::Tokens;
use std::mem;
use std::process;

fn is_same_variant(a: &Tokens, b: &Tokens) -> bool {
    mem::discriminant(a) == mem::discriminant(b)
}

pub struct Parser {
    lexer: Lexer,
    symbol_table: SymbolTable,
    codegen: Codegen,
    current_token: Tokens,
}

impl Parser {
    pub fn new(source_code: String) -> Self {
        let mut lexer = Lexer::new(source_code);

        let initial_token = lexer.next_token();

        Self {
            lexer,
            symbol_table: SymbolTable::new(),
            codegen: Codegen::new("output.c".to_string()),
            current_token: initial_token,
        }
    }

    fn current(&self) -> &Tokens {
        &self.current_token
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, expected_tok: &Tokens) -> Tokens {
        let current_tok = self.current();

        let matches = if expected_tok == current_tok || is_same_variant(expected_tok, current_tok) {
            true
        } else if is_same_variant(expected_tok, current_tok) {
            true
        } else {
            false
        };

        if !matches {
            eprintln!(
                "ERROR on line {}: Expected {:?} but got: {:?}",
                self.lexer.line, expected_tok, current_tok
            );
            process::exit(1);
        }

        let consumed_token = current_tok.clone();
        self.advance();

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
                Tokens::IDENT(_) => self.parse_fn_decl(),
                _ => {
                    eprintln!(
                        "ERROR on line {}: Unexpected token in global scope: {:?}",
                        self.lexer.line,
                        self.current()
                    );
                    process::exit(1);
                }
            }
        }
        self.codegen.end();
    }

    fn parse_fn_decl(&mut self) {
        let func_name = self.consume_ident_value();
        self.expect(&Tokens::DOUBLECOL);

        let func_ret_name = self.consume_ident_value();
        let func_ret_type = match func_ret_name.as_str() {
            "void" => FunctionType::VOID,
            _ => {
                eprintln!(
                    "ERROR on line {}: Unknow function return type: {}",
                    self.lexer.line, func_ret_name
                );
                process::exit(1);
            }
        };

        self.expect(&Tokens::OPENCURLY);

        self.codegen.start_function(&func_name, &func_ret_type);
        self.symbol_table.set_func(func_name, func_ret_type);
        self.parse_stmt();
        self.expect(&Tokens::CLOSECURLY);
    }

    fn parse_stmt(&mut self) {
        loop {
            let current = self.current();
            if current == &Tokens::CLOSECURLY {
                self.codegen.end_function();
                break;
            }

            if current == &Tokens::EOF {
                eprintln!(
                    "ERROR on line {}: Expected '}}' but reached end of file.",
                    self.lexer.line
                );
                process::exit(1);
            }

            match current {
                Tokens::LET => {
                    self.parse_let_stmt();
                }
                _ => {
                    eprintln!(
                        "ERROR on line {}: Unexpected token in function scope: {:?}",
                        self.lexer.line, current
                    );

                    self.advance();
                    process::exit(1);
                }
            }
        }
    }

    fn parse_let_stmt(&mut self) {
        self.expect(&Tokens::LET);
        let var_name = self.consume_ident_value();
        self.expect(&Tokens::EQUALS);

        let var_type: VariableType;
        match self.current() {
            Tokens::NUMBER(n) => {
                var_type = VariableType::INT32(*n);
                self.advance();
            }
            _ => {
                eprintln!(
                    "ERROR on line {}: Expected a number or expression but got: {:?}",
                    self.lexer.line,
                    self.current()
                );
                process::exit(1);
            }
        }

        self.expect(&Tokens::SEMICOLON);
        self.codegen.let_stmt(&var_name, &var_type);
        self.symbol_table.set_var(var_name, var_type);
    }
}
