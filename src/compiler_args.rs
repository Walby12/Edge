use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CompilerArgs {
    #[arg(required = true, value_name = "FILE")]
    pub input: PathBuf,
    #[arg(short, long, value_name = "OUTPUT")]
    pub output: Option<PathBuf>,
    #[arg(short, long)]
    pub debug: bool,
}
