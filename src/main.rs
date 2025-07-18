// src/main.rs
/*
 * Main executable for ArtificialMintMasterKitUltra
 */

use clap::Parser;
use artificialmintmasterkitultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialMintMasterKitUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
