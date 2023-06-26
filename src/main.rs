mod parser;

use anyhow::Result;
use parser::*;

use clap::Parser;

/// Simple program to convert from .sos -> .dat
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to sos file
    #[arg(short, long)]
    filename: String,

    /// Path to .dat directory
    #[arg(short, long)]
    outdir: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let filename = args.filename;
    let outdir = args.outdir;

    // Check if files can be parsed
    if filename.split(".").last().unwrap() != ".sos" {
        panic!("Incorrect file extensions. Input file should be *.sos");
    }

    // Parse file
    match parse(filename, outdir) {
        Ok(_) => {
            println!("Finished parsing file");
            Ok(())
        }
        Err(e) => panic!("{}", e),
    }
}
