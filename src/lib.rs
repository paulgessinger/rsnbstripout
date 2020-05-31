use serde::Serialize;
use serde_json::{json, Value};
use std::io;

fn do_strip(mut json: Value) -> Value {
    let mut _cells = json.get_mut("cells").unwrap();

    if let Value::Array(cells) = &mut _cells {
        for cell in cells {
            if let Some(outputs) = cell.get_mut("outputs") {
                *outputs = json!([]);
            }

            if let Some(metadata) = cell.get_mut("metadata") {
                *metadata = json!({});
            }

            if let Some(execution_count) = cell.get_mut("execution_count") {
                *execution_count = Value::Null;
            }
        }
    }
    return json;
}

pub fn strip_pipe<R: io::Read, W: io::Write>(read: R, write: W) -> serde_json::Result<()> {
    let raw: Value = serde_json::from_reader(read)?;

    let processed = do_strip(raw);

    let fmt = serde_json::ser::PrettyFormatter::with_indent(b"  ");
    let mut ser = serde_json::ser::Serializer::with_formatter(write, fmt);
    processed.serialize(&mut ser)?;

    // let mut write = ser.into_inner();
    // write.write(b"\n");

    return Ok(());
}

#[allow(dead_code)]
pub fn strip_string(input: &str) -> serde_json::Result<String> {
    let reader = io::BufReader::new(input.as_bytes());
    let buf = Vec::new();
    let mut writer = io::BufWriter::new(buf);

    strip_pipe(reader, &mut writer)?;
    Ok(String::from_utf8_lossy(writer.buffer()).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Read, Write};
    use std::path::Path;

    fn get_raw() -> &'static str {
        r###"
{
  "cells": [
    {
      "cell_type": "markdown",
      "metadata": {},
      "source": [
        "# Running Code"
      ]
    },
    {
      "cell_type": "code",
      "execution_count": 2,
      "metadata": {
        "collapsed": false
      },
      "outputs": [],
      "source": [
        "a = 10"
      ]
    },
    {
      "cell_type": "code",
      "execution_count": 3,
      "metadata": {
        "collapsed": false
      },
      "outputs": [
        {
          "name": "stdout",
          "output_type": "stream",
          "text": [
            "10\n"
          ]
        }
      ],
      "source": [
        "print(a)"
      ]
    }
  ]
}
    "###
        .trim()
    }

    fn get_exp() -> &'static str {
        r###"
{
  "cells": [
    {
      "cell_type": "markdown",
      "metadata": {},
      "source": [
        "# Running Code"
      ]
    },
    {
      "cell_type": "code",
      "execution_count": null,
      "metadata": {},
      "outputs": [],
      "source": [
        "a = 10"
      ]
    },
    {
      "cell_type": "code",
      "execution_count": null,
      "metadata": {},
      "outputs": [],
      "source": [
        "print(a)"
      ]
    }
  ]
}
  "###.trim()
    }

    #[test]
    fn test_strip() {
        let raw = get_raw();
        let exp = get_exp();

        let parsed_raw: Value = serde_json::from_str(raw).expect("Unable to parse JSON");
        let act_json = do_strip(parsed_raw);
        let act = serde_json::to_string_pretty(&act_json).unwrap();
        assert_eq!(act, exp);
    }

    #[test]
    fn test_strip_pipe() {
        let raw = get_raw();
        let exp = get_exp();

        let reader = io::BufReader::new(raw.as_bytes());
        let buf = Vec::new();
        let mut writer = io::BufWriter::new(buf);

        strip_pipe(reader, &mut writer).expect("strip pipe failed");
        let act = String::from_utf8_lossy(writer.buffer());
        assert_eq!(act, exp);
    }

    #[test]
    fn test_strip_file() {
        let raw = get_raw();
        let exp = get_exp();

        let reader = io::BufReader::new(raw.as_bytes());
        let buf = Vec::new();
        let mut writer = io::BufWriter::new(buf);

        strip_pipe(reader, &mut writer).expect("strip pipe failed");
        let act = String::from_utf8_lossy(writer.buffer());
        assert_eq!(act, exp);
    }

    #[test]
    fn test_strip_string() {
        let raw = get_raw();
        let exp = get_exp();

        let act = strip_string(raw).expect("strip_string failed");
        assert_eq!(act, exp);
    }

    #[test]
    fn test_strip_files() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let infile = manifest_dir.join("raw.ipynb");

        println!("{}", infile.display());
        assert!(infile.exists());

        let fh = File::open(infile).expect("file couldn't be opened");
        let read = BufReader::new(fh);

        let expfile = manifest_dir.join("stripped.ipynb");
        let mut expfh = File::open(&expfile).expect("ref file read failed");
        let mut exp = String::new();
        expfh
            .read_to_string(&mut exp)
            .expect("ref file read failed");

        // println!("{}", exp);

        let outfile = std::env::temp_dir().join("rsnbstripout_test_act.ipynb");
        if outfile.exists() {
            std::fs::remove_file(&outfile).expect("outfile removal failed");
        }

        let outfh = File::create(&outfile).expect("output file not created");
        let mut writer = BufWriter::new(outfh);

        strip_pipe(read, &mut writer).expect("strip pipe failed");

        writer.write(b"\n").expect("unable to add newline");
        writer.flush().expect("unable to flush");
        drop(writer);

        let mut actfh = File::open(outfile).expect("ref file read failed");
        let mut act = String::new();
        actfh
            .read_to_string(&mut act)
            .expect("act file read failed");

        assert_eq!(act, exp);
    }
}
