mod lib;

use std;
use std::boxed::Box;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // for arg in args.iter() {
    //   println!("{}", arg);
    // }

    // let reader: Option<Read> = None;
    let reader: Option<Box<dyn Read>>;
    let writer: Option<Box<dyn Write>>;
    // let reader: Option<&dyn Read>;
    // let writer: Option<&dyn Write>;

    if args.len() == 0 {
        // stdin -> stdout
        reader = Some(Box::new(std::io::stdin()));
        writer = Some(Box::new(std::io::stdout()));
    } else {
        if args.len() > 2 {
            panic!("Too many arguments ({}). Don't know what to do", args.len());
        }
        // either one or two args
        let infile: &str = &args[0];
        let fh = match File::open(&infile) {
            Ok(v) => v,
            Err(e) => panic!("Unable to open {}: {}", infile, e),
        };
        reader = Some(Box::new(BufReader::new(fh)));

        if args.len() == 2 {
            let outfile = &args[1];

            let fh = match File::create(&outfile) {
                Ok(v) => v,
                Err(e) => panic!("Unable to create file at {}: {}", outfile, e),
            };

            writer = Some(Box::new(BufWriter::new(fh)));
        } else {
            writer = Some(Box::new(std::io::stdout()));
        }
    }

    let reader = reader.unwrap();
    let mut writer = writer.unwrap();

    match lib::strip_pipe(reader, &mut writer) {
        Err(err) => println!("There was an error: {}", err),
        Ok(_) => {}
    }
}
