use std::{fs::File, path::Path};

use anyhow::Result;

/// Read input from file
fn prepare_inp_data(infile: String) -> Result<String> {
    let path = Path::new(&infile);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(e) => panic!("Could not open {}: {}", display, e),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        res => res,
    }
}

fn prepare_out_data(out_file: String) -> Result<File> {
    let path = Path::new(&format!("./files/{out_file}"));
    let display = path.display();

    match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => Ok(file),
    }
}

/// function parses a sos file to a .dat file
pub fn parse(in_file: String, out_file: String) -> Result<()> {
    let data = match prepare_inp_data(in_file) {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };

    let file = match prepare_out_data(out_file) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    Ok(())
}
