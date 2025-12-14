mod codegen;
mod compiler_args;
mod lexer;
mod parser;
mod symbol_table;
mod tokens;
use crate::compiler_args::CompilerArgs;
use crate::parser::parser::Parser;
use clap::Parser as ClapParser;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::time::Instant;

fn main() {
    let args = CompilerArgs::parse();

    let source_path = args.input;

    if source_path.extension().map_or(true, |ext| ext != "ed") {
        eprintln!(
            "ERROR: Source file must end with the '.ed' extension. Found: {}",
            source_path.display()
        );
        process::exit(1);
    }

    let output_path = args.output.unwrap_or_else(|| {
        source_path
            .file_stem()
            .unwrap_or_else(|| "a.out".as_ref())
            .to_string_lossy()
            .to_string()
            .replace(".", "")
            .into()
    });

    let output_path_c = PathBuf::from(format!("{}.c", output_path.display()));

    if args.debug {
        println!("*** DEBUG MODE ENABLED ***");
        println!("Source file: {}", source_path.display());
        println!("Target C file: {}", output_path_c.display());
    }

    let source_code = match fs::read_to_string(&source_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error reading source file {}: {}", source_path.display(), e);
            process::exit(1);
        }
    };

    let mut parser = Parser::new(source_code, output_path_c.to_string_lossy().into_owned());

    println!("Compiling {}...", source_path.display());

    let start_time = Instant::now();

    parser.parse();

    let duration = start_time.elapsed();
    let total_seconds = duration.as_secs();
    let milliseconds = duration.subsec_millis();

    println!(
        "Compilation finished in: {}.{:03}s",
        total_seconds, milliseconds
    );
}
