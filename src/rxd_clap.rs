use clap::builder::ValueParserFactory;
use clap::Parser;

use std::path::Path;

#[derive(Parser)]
#[command(
    bin_name = "rxd",
    author = "StoatSec",
    version = "1.0",
    about = "rxd: a hex dump written in rust",
)]
pub struct Args {
    /// Target file path
    #[arg(value_parser = String::value_parser())]
    pub file_path: String,

    /// Dictates whether to show binary or hex (including the flag sets format to binary)
    #[arg(short, long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub binary: bool,

    /// How much of the file (in bytes) to load into memory at once
    // regex search will also only apply to once chunk at a time
    // meaning there is a potential blindspot for string searches between chunks
    // (will probably address this in future updates)
    #[arg(short, long, default_value = "4096")]
    pub chunks: usize,

    /// Regex pattern to match ascii output against
    #[arg(short, long)]
    pub pattern: Option<String>,
}

pub fn parse_args() -> Result<Args, Box<dyn std::error::Error>> {
    let args = Args::parse();

    let path = &args.file_path;

    let p = Path::new(path.as_str());
    if !p.exists() {
        return Err(format!("file '{}' does not exist", path).into());
    }

    if &args.chunks <= &0 {
        return Err(format!("chunks arg must be greater than zero").into());
    }

    return Ok(args);
}