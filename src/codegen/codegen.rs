use crate::symbol_table::{FunctionType, VariableType};
use std::fs;

pub struct Codegen {
    file_name: String,
    builder: String,
}

impl Codegen {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name: file_name,
            builder: String::new(),
        }
    }

    pub fn end(&self) {
        fs::write(&self.file_name, &self.builder).unwrap();
    }

    pub fn let_stmt(&mut self, var_name: &str, var_type: &VariableType) {
        let (r#type, value) = match var_type {
            VariableType::INT32(n) => ("int", n),
        };

        let str = format!("\t{} {} = {};\n", r#type, var_name, value);
        self.builder.push_str(&str);
    }

    pub fn var_reassign(&mut self, var_name: &str, var_type: &VariableType) {
        let (_, value) = match var_type {
            VariableType::INT32(n) => ("", n),
        };

        let str = format!("\t{} = {};\n", var_name, value);
        self.builder.push_str(&str);
    }

    pub fn start_function(&mut self, func_name: &str, func_type: &FunctionType) {
        let r#type = match func_type {
            FunctionType::VOID => "void",
        };

        let str = format!("{} {} {{\n", r#type, func_name);
        self.builder.push_str(&str)
    }

    pub fn end_function(&mut self) {
        self.builder.push_str("}\n");
    }
}
