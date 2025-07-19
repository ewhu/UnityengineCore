// src/main.rs
/*
 * Main executable for UnityengineCore
 */

use clap::Parser;
use unityenginecore::{Result, run};

#[derive(Parser)]
#[command(version, about = UnityengineCore - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
