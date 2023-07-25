use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Failed to read {}", args.path.to_str().unwrap()) )?;
    grrs::print_matches(&args.pattern, &content, &mut std::io::stdout())?;
    Ok(())
}

