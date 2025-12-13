use crate::symbol_table::VariableType;

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

    pub fn let_stmt(&mut self, var_name: &str, var_type: &VariableType) {
        let (r#type, value) = match var_type {
            VariableType::INT32(n) => ("int", n),
        };

        let str = format!("{} {} = {};", r#type, var_name, value);
        self.builder.push_str(&str);
        println!("{}", self.builder);
    }
}
