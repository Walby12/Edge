use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum VariableType {
    INT32(i32),
}

pub struct SymbolTable {
    vars: HashMap<String, VariableType>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Result<VariableType, String> {
        self.vars
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Variable '{}' not declared.", name))
    }

    pub fn set(&mut self, name: String, value: VariableType) {
        self.vars.insert(name, value);
    }

    pub fn print_state(&self) {
        println!("\n--- Current Symbol Table State ---");
        if self.vars.is_empty() {
            println!("(Empty)");
        } else {
            for (name, value) in &self.vars {
                println!("{}: {:?}", name, value);
            }
        }
        println!("----------------------------------");
    }
}
