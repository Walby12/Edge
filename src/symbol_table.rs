use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum VariableType {
    INT32(i32),
}

#[derive(Clone, Debug)]
pub enum FunctionType {
    VOID,
    INT,
}

pub struct SymbolTable {
    vars: HashMap<String, VariableType>,
    functions: HashMap<String, FunctionType>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            vars: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get_var(&self, name: &str) -> Result<VariableType, String> {
        self.vars
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Variable '{}' not in scope", name))
    }

    pub fn set_var(&mut self, name: String, value: VariableType) {
        self.vars.insert(name, value);
    }

    pub fn get_func(&self, name: &str) -> Result<FunctionType, String> {
        self.functions
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Function '{}' is not defined", name))
    }

    pub fn set_func(&mut self, name: String, value: FunctionType) {
        self.functions.insert(name, value);
    }
}
