use std::fs::read_to_string;
use std::io::Write;
use std::str::FromStr;
use std::{fs::File, path::Path};

use anyhow::Result;

/// function parses a sos file to a .dat file
#[warn(unused_assignments)]
pub fn parse(in_file: String, out_dir: String) -> Result<()> {
    // Filename for out file is the last part of input name
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
    let display = inpath.display();

    let file_contents = match read_to_string(inpath) {
        Ok(contents) => contents,
        Err(err) => {
            panic!("Failed to read file: {}, err: {}", display, err);
        }
    };

    let origone = [0.0, 0.0];
    let enhet = 0.01;
    let mut mm = 0;
    let dm = 1.4;

    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut d = Vec::new();

    let mut d_no = 0.0;

    for line in file_contents.lines() {
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

            for line_depth in file_contents.lines() {
                if line_depth.trim_start().starts_with('.') {
                    break;
                }

                let input = line_depth.trim();
                let n_o: Vec<&str> = input.split_whitespace().collect();

                if n_o.len() < 2 {
                    continue;
                }

                mm += 1;
                if let (Ok(x_coord), Ok(y_coord)) = (f64::from_str(n_o[1]), f64::from_str(n_o[0])) {
                    x.push(x_coord);
                    y.push(y_coord);
                    d.push(d_no);
                }
            }
        } else if line.contains("Kystkontur") {
            let d_ky = dm;

            for line_coast in file_contents.lines() {
                if line_coast.trim_start().starts_with('.') {
                    break;
                }

                let input = line_coast.trim();
                let n_o: Vec<&str> = input.split_whitespace().collect();

                if n_o.len() < 2 {
                    continue;
                }

                mm += 1;
                if let (Ok(x_coord), Ok(y_coord)) = (f64::from_str(n_o[1]), f64::from_str(n_o[0])) {
                    x.push(x_coord);
                    y.push(y_coord);
                    d.push(d_ky);
                }
            }
        }
    }

    let mut nxyd = vec![[0.0; 4]; mm];

    for m in 0..mm {
        nxyd[m][0] = (m + 1) as f64;
        nxyd[m][1] = x[m] * enhet + origone[1];
        nxyd[m][2] = y[m] * enhet + origone[0];
        nxyd[m][3] = d[m];
    }

    let out_path = Path::new(&out_dir).join(out_file);

    println!("{:?}", out_path);

    // Writing to output file
    let mut geo_file = match File::create(out_path) {
        Ok(file) => file,
        Err(err) => {
            panic!("Failed to create file: {}", err);
        }
    };

    for row in &nxyd {
        let line = format!("{:.6} {:.6} {:.6}\n", row[1], row[2], row[3]);
        if let Err(err) = geo_file.write_all(line.as_bytes()) {
            panic!("Failed to write to file: {}", err);
        }
    }

    Ok(())
}
