use std::io::Read;
use std::{fs::File, path::Path};

use anyhow::Result;

/// function parses a sos file to a .dat file
pub fn parse(in_file: String, out_dir: String) -> Result<()> {
    // Filename for out file is the last part of input name
    let inpath = Path::new(&in_file);

    // name of output file will be the same as input file
    let out_file: String = inpath
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
        .split(".")
        .nth(0)
        .unwrap()
        .to_owned();

    let display = inpath.display();

    // Open .sos file
    let mut file = match File::open(&inpath) {
        Err(e) => panic!("Could not open {}: {}", display, e),
        Ok(file) => file,
    };

    // TODO: Too big of a string to store on heap?
    let mut s = String::new();
    let data: String = match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => s,
    };

    // ============================================
    // Out data
    // ============================================

    let path = Path::new(&out_dir).join(out_file);
    let display = path.display();

    let mut f = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    for l in data.lines() {}

    Ok(())
}

fn a() {}

fn b() {}
