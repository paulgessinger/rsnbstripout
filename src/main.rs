#![cfg_attr(feature = "strict", deny(warnings))]

mod lib;

use std;
use std::boxed::Box;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

fn get_reader_writer(args: &Vec<String>) -> Result<(Box<dyn Read>, Box<dyn Write>), String> {
    if args.len() == 0 {
        // stdin -> stdout
        return Ok((Box::new(std::io::stdin()), Box::new(std::io::stdout())));
    }

    if args.len() > 2 {
        return Err(format!(
            "Too many arguments ({}). Don't know what to do",
            args.len()
        ));
    }

    let infile: &str = &args[0];
    let fh = match File::open(&infile) {
        Ok(v) => v,
        Err(e) => return Err(format!("Unable to open {}: {}", infile, e)),
    };
    let reader = Box::new(BufReader::new(fh));

    if args.len() < 2 {
        return Ok((reader, Box::new(std::io::stdout())));
    }

    let outfile = &args[1];

    let fh = match File::create(&outfile) {
        Ok(v) => v,
        Err(e) => return Err(format!("Unable to create file at {}: {}", outfile, e)),
    };

    Ok((reader, Box::new(BufWriter::new(fh))))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let (reader, mut writer) = match get_reader_writer(&args) {
        Ok((r, w)) => (r, w),
        Err(err) => panic!("{}", err),
    };

    match lib::strip_pipe(reader, &mut writer) {
        Err(err) => panic!("There was an error: {}", err),
        Ok(_) => {}
    }
}
