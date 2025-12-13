use crate::symbol_table::SymbolTable;
use crate::tokens::Tokens;

pub struct Parser {
    symbol_table: SymbolTable,
    toks: Vec<Tokens>,
    index: usize,
    line: usize,
}

impl Parser {
    pub fn new(t: Vec<Tokens>) -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            toks: t,
            index: 0,
            line: 1,
        }
    }

    pub fn parse(&mut self) {
        while *self.toks.get(self.index).unwrap() != Tokens::EOF {
            match *self.toks.get(self.index).unwrap() {
                Tokens::LET => self.parse_let_stmt(),
                _ => todo!(),
            }
            self.index += 1;
        }
    }

    fn parse_let_stmt(&mut self) {
        println!("LET STMT");
    }
}
