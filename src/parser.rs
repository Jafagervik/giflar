use rayon::prelude::*;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;
use std::{fs::File, path::Path};

use anyhow::Result;

#[warn(unused_assignments)]
pub fn parse(in_file: String, out_dir: String) -> Result<()> {
    let inpath = Path::new(&in_file);
    println!("{:?}", inpath);

    // name of output file will be the same as input file
    let mut out_file: String = inpath
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
        .split(".")
        .nth(0)
        .unwrap()
        .to_owned();

    out_file.push_str(".dat");

    // Open .sos file
    let infile = File::open(inpath)?;
    let reader = BufReader::new(infile.try_clone()?);

    // Clone file to avoid reading all contents into memory

    // let origone = [0.0, 0.0]; Not needed
    let scale = 0.01;
    let mut mm = 0;
    let dm = 1.4;

    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut d = Vec::new();

    let mut d_no = 0.0;

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        if line.contains("DYBDE") {
            if let Some(stripped_line) = line.splitn(2, ' ').nth(1) {
                if let Ok(parsed_d_no) = f64::from_str(stripped_line) {
                    d_no = parsed_d_no;
                } else {
                    continue;
                }
            } else {
                continue;
            }

            mm += inner(infile.try_clone()?, d_no, &mut x, &mut y, &mut d);
        } else if line.contains("Kystkontur") {
            mm += inner(infile.try_clone()?, dm, &mut x, &mut y, &mut d);
        }
    }

    let nxyd: Vec<[f64; 4]> = (0..mm)
        .into_par_iter()
        .map(|n| [(n + 1) as f64, x[n] * scale, y[n] * scale, d[n]])
        .collect();

    let out_path = Path::new(&out_dir).join(out_file);

    println!("{:?}", out_path);

    let mut out_file = match File::create(out_path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Failed to create file: {}", e);
        }
    };

    // Writing to output file
    for row in &nxyd {
        let line = format!("{:.6} {:.6} {:.6}\n", row[1], row[2], row[3]);
        if let Err(e) = out_file.write_all(line.as_bytes()) {
            panic!("Failed to write to file: {}", e);
        }
    }

    Ok(())
}

/// Inner loop for the parser
fn inner(
    f: File,
    val: f64,
    // mut mm: usize,
    x: &mut Vec<f64>,
    y: &mut Vec<f64>,
    d: &mut Vec<f64>,
) -> usize {
    let mut mm = 0;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim_start().starts_with('.') {
            break;
        }

        let n_o: Vec<&str> = line.trim().split_whitespace().collect();

        if n_o.len() < 2 {
            continue;
        }

        mm += 1;
        if let (Ok(x_coord), Ok(y_coord)) = (f64::from_str(n_o[1]), f64::from_str(n_o[0])) {
            x.push(x_coord);
            y.push(y_coord);
            d.push(val);
        }
    }
    return mm;
}
