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

    /// Output file
    #[arg(short, long, default_value_t = String::from("output.dat"))]
    out_filename: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let filename = args.filename;
    let out_filename = args.out_filename;

    // Check if files can be parsed
    if filename.split(".").last().unwrap() != ".sos"
        || out_filename.split(".").last().unwrap() != ".dat"
    {
        panic!("Incorrect file extensions. Input file should be *.sos, and output: *.dat");
    }

    // Parse file
    match parse(filename, out_filename) {
        Ok(_) => {
            println!("Finished parsing file");
            Ok(())
        }
        Err(e) => panic!("{}", e),
    }
}
