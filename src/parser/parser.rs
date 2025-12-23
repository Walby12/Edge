use crate::codegen::codegen::Codegen;
use crate::lexer::lexer::Lexer;
use crate::symbol_table::{FunctionType, SymbolTable, VariableType};
use crate::tokens::Tokens;
use std::mem;
use std::process;

fn is_same_variant(a: &Tokens, b: &Tokens) -> bool {
    mem::discriminant(a) == mem::discriminant(b)
}

fn tok_to_string(a: &Tokens) -> String {
    match a {
        Tokens::NUMBER(n) => format!("{}", n),
        Tokens::IDENT(n) => format!("ident {}", n),
        Tokens::STRING(n) => format!("string {}", n),
        Tokens::SEMICOLON => ";".to_string(),
        Tokens::EQUALS => "=".to_string(),
        Tokens::LET => "let".to_string(),
        Tokens::EOF => "end of file".to_string(),
        Tokens::PLUS => "+".to_string(),
        Tokens::MINUS => "-".to_string(),
        Tokens::DOUBLECOL => "::".to_string(),
        Tokens::OPENCURLY => "{".to_string(),
        Tokens::CLOSECURLY => "}".to_string(),
        Tokens::CLOSEPAREN => ")".to_string(),
        Tokens::OPENPAREN => "(".to_string(),
        Tokens::VOID => "void type".to_string(),
        Tokens::INT => "int type".to_string(),
        Tokens::RETURN => "return ".to_string(),
    }
}

pub struct Parser {
    lexer: Lexer,
    symbol_table: SymbolTable,
    codegen: Codegen,
    current_token: Tokens,
    has_return: bool,
}

impl Parser {
    pub fn new(source_code: String, out_file_name: String) -> Self {
        let mut lexer = Lexer::new(source_code);

        let initial_token = lexer.next_token();

        Self {
            lexer,
            symbol_table: SymbolTable::new(),
            codegen: Codegen::new(out_file_name),
            current_token: initial_token,
            has_return: false,
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
                "ERROR on line {}: Expected {} but got: {}",
                self.lexer.line,
                tok_to_string(expected_tok),
                tok_to_string(current_tok)
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
                Tokens::IDENT(n) => {
                    if n == "c_comp_append" {
                        self.parse_c_com_append();
                    } else {
                        self.parse_fn_decl();
                    }
                }
                _ => {
                    eprintln!(
                        "ERROR on line {}: Unexpected token in global scope: {}",
                        self.lexer.line,
                        tok_to_string(self.current())
                    );
                    process::exit(1);
                }
            }
        }
        self.codegen.end();
    }

    fn parse_c_com_append(&mut self) {
        self.advance();
        self.expect(&Tokens::OPENPAREN);

        let current = self.current();
        match current {
            Tokens::STRING(n) => self.codegen.c_comp_append(n.to_string()),
            Tokens::IDENT(_n) => todo!(),
            _ => {
                eprintln!(
                    "ERROR on line {}: Expected a string or an ident for c_comp_append but got: {}",
                    self.lexer.line,
                    tok_to_string(current)
                );
                process::exit(1);
            }
        }
        self.advance();
        self.expect(&Tokens::CLOSEPAREN);
        self.expect(&Tokens::SEMICOLON);
    }

    fn parse_fn_decl(&mut self) {
        let func_name = self.consume_ident_value();
        self.expect(&Tokens::DOUBLECOL);

        let current = self.current();
        let func_ret_type = match current {
            Tokens::VOID => {
                if func_name == "main" {
                    eprintln!(
                        "ERROR on line {}: Return type of main must be int got void",
                        self.lexer.line
                    );
                    process::exit(1);
                }
                FunctionType::VOID
            }
            Tokens::INT => FunctionType::INT,
            _ => {
                eprintln!(
                    "ERROR on line {}: Unknow return type: {}, expected 'void', 'int'",
                    self.lexer.line,
                    tok_to_string(current)
                );
                process::exit(1);
            }
        };
        self.advance();

        self.expect(&Tokens::OPENCURLY);

        self.codegen.start_function(&func_name, &func_ret_type);
        self.symbol_table.set_func(func_name, func_ret_type.clone());
        self.parse_stmt(func_ret_type.clone());
        if !self.has_return {
            eprintln!(
                "ERROR on line {}: Expected return a the end of a function",
                self.lexer.line
            );
            process::exit(1);
        }
        self.advance();
    }

    fn parse_stmt(&mut self, func_ret_type: FunctionType) {
        loop {
            let token_type = mem::discriminant(self.current());

            if token_type == mem::discriminant(&Tokens::CLOSECURLY) {
                self.codegen.end_function();
                break;
            }

            if token_type == mem::discriminant(&Tokens::EOF) {
                eprintln!(
                    "ERROR on line {}: Expected '}}' but reached end of file.",
                    self.lexer.line
                );
                process::exit(1);
            }

            match self.current() {
                Tokens::LET => {
                    self.parse_let_stmt();
                }

                Tokens::IDENT(name) => {
                    let action_name = name.clone();

                    self.advance();

                    match self.current() {
                        Tokens::EQUALS => {
                            self.parse_var_reassign(action_name);
                        }
                        Tokens::OPENPAREN => {
                            self.parse_func_call(action_name);
                        }
                        _ => {
                            eprintln!(
                                "ERROR on line {}: Expected '=' or '(' but got: {}",
                                self.lexer.line,
                                tok_to_string(self.current())
                            );
                            process::exit(1);
                        }
                    }
                }
                Tokens::RETURN => {
                    match func_ret_type {
                        FunctionType::INT => {
                            self.advance();
                            let curr = self.current();
                            match curr {
                                Tokens::NUMBER(n) => {
                                    self.codegen.return_stmt(n.to_string());
                                }
                                Tokens::IDENT(n) => match self.symbol_table.get_var(n) {
                                    Ok(t) => {
                                        if mem::discriminant(&t)
                                            != mem::discriminant(&VariableType::INT32(
                                                0.to_string(),
                                            ))
                                        {
                                            eprintln!("ERROR on line {}: Cannot return a non int var from a int function", self.lexer.line);
                                            process::exit(1);
                                        } else {
                                            self.codegen.return_stmt(n.to_string());
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("ERROR on line {}: {}", self.lexer.line, e);
                                        process::exit(1);
                                    }
                                },
                                _ => {
                                    eprintln!("ERROR on line {}: Expected a number or an ident at the end of a int returning function but got: {}", self.lexer.line, tok_to_string(curr));
                                    process::exit(1);
                                }
                            }
                        }
                        FunctionType::VOID => self.codegen.return_stmt("".to_string()),
                    }
                    self.advance();
                    self.expect(&Tokens::SEMICOLON);

                    let curr = self.current();
                    match curr {
                        Tokens::CLOSECURLY => {
                            self.has_return = true;
                            self.codegen.end_function();
                            break;
                        }
                        _ => {
                            eprintln!(
                                "ERROR at line {}: Useless stmts after return stmt",
                                self.lexer.line
                            );
                            process::exit(1);
                        }
                    }
                }
                _ => {
                    eprintln!(
                        "ERROR on line {}: Unexpected token in function scope: {}",
                        self.lexer.line,
                        tok_to_string(self.current())
                    );

                    self.advance();
                    process::exit(1);
                }
            }
        }
    }

    fn parse_func_call(&mut self, name: String) {
        if name == "putchar".to_string() {
            self.expect(&Tokens::OPENPAREN);
            match self.current() {
                Tokens::NUMBER(n) => {
                    let value = n.clone();
                    self.advance();
                    self.expect(&Tokens::CLOSEPAREN);
                    self.codegen.function_call(name, value.to_string());
                }
                _ => {
                    eprintln!(
                        "ERROR at line {}: Expected a number value for function putchar got: {}",
                        self.lexer.line,
                        tok_to_string(self.current())
                    );
                    process::exit(1);
                }
            }
        } else {
            eprintln!(
                "ERROR at line {}: Unknow function: {}",
                self.lexer.line, name
            );
            process::exit(1);
        }
        self.expect(&Tokens::SEMICOLON);
    }

    fn parse_var_reassign(&mut self, name: String) {
        self.expect(&Tokens::EQUALS);
        let var = self.symbol_table.get_var(&name);

        match var {
            Ok(declared_type) => match declared_type {
                VariableType::INT32(_) => match self.current() {
                    Tokens::NUMBER(v) => {
                        let var_type = VariableType::INT32((*v).to_string());
                        self.codegen.var_reassign(&name, &var_type);
                        self.advance();
                    }
                    _ => {
                        eprintln!(
                            "ERROR on line {}: Expected a number or an ident but got: {}",
                            self.lexer.line,
                            tok_to_string(self.current())
                        );
                        process::exit(1);
                    }
                },
            },
            Err(e) => {
                eprintln!("ERROR on line {}: {}", self.lexer.line, e);
                process::exit(1);
            }
        }
        self.expect(&Tokens::SEMICOLON);
    }

    fn parse_let_stmt(&mut self) {
        self.expect(&Tokens::LET);
        let var_name = self.consume_ident_value();
        self.expect(&Tokens::EQUALS);

        let var_type: VariableType;
        match self.current() {
            Tokens::NUMBER(n) => {
                var_type = VariableType::INT32((*n).to_string());
                self.advance();
            }
            Tokens::IDENT(n) => match self.symbol_table.get_var(n) {
                Ok(t) => {
                    var_type = match t {
                        VariableType::INT32(_) => VariableType::INT32(n.to_string()),
                    };
                    self.advance();
                }
                Err(e) => {
                    eprintln!("ERROR on line {}: {}", self.lexer.line, e);
                    process::exit(1);
                }
            },
            _ => {
                eprintln!(
                    "ERROR on line {}: Expected a number or expression but got: {}",
                    self.lexer.line,
                    tok_to_string(self.current())
                );
                process::exit(1);
            }
        }

        self.expect(&Tokens::SEMICOLON);
        self.codegen.let_stmt(&var_name, &var_type);
        self.symbol_table.set_var(var_name, var_type);
    }
}
