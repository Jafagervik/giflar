mod parser;

use std::{fs, panic};

use anyhow::Result;
use parser::parse;

use clap::Parser;

/// Simple program to convert from .sos -> .dat
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to sos file or directory
    #[arg(short, long)]
    inpath: String,

    /// Path to .dat directory
    #[arg(short, long)]
    outdir: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Check if file can be parsed
    if args.inpath.contains(".sos") {
        match parse(args.inpath, args.outdir) {
            Ok(_) => {
                println!("Finished parsing file");
                return Ok(());
            }
            Err(e) => panic!("{}", e),
        };
    }

    // In a directory, try parsing all .sos files
    let mut parsed = 0;
    let mut tot_sosi_files = 0;

    match fs::read_dir(args.inpath.clone()) {
        Err(e) => panic!("Error: {}", e),
        Ok(files) => {
            for f in files {
                let f = f.unwrap().path().to_string_lossy().to_string();
                if f.contains(".sos") {
                    match parse(f.clone(), args.outdir.clone()) {
                        Ok(_) => {
                            println!("Finished parsing file {}", f);
                            parsed += 1;
                        }
                        Err(e) => {
                            eprintln!("Error parsing that file, jumping to next. {}", e);
                        }
                    };
                    tot_sosi_files += 1;
                }
            }
        }
    }

    println!(
        "Finished parsing {} out of {} .sos files in directory {}",
        parsed, tot_sosi_files, args.inpath
    );

    Ok(())
}
