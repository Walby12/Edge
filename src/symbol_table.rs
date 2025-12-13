use std::collections::HashMap;

pub struct SymbolTable {
    vars: HashMap<String, i32>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable { vars: HashMap::new() }
    }

    pub fn get(&self, name: &str) -> Result<i32, String> {
        self.vars.get(name).cloned().ok_or_else(|| {
            format!("Runtime Error: Variable '{}' not declared.", name)
        })
    }

    pub fn set(&mut self, name: String, value: i32) {
        self.vars.insert(name, value);
    }
    
    pub fn print_state(&self) {
        println!("\n--- Current Symbol Table State ---");
        if self.vars.is_empty() {
            println!("(Empty)");
        } else {
            for (name, value) in &self.vars {
                println!("{}: {}", name, value);
            }
        }
        println!("----------------------------------");
    }
}
