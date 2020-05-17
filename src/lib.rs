use serde_json::{json, Value};
use std::string::String;

pub fn strip(input: &str) -> String {
  let mut parsed: Value = serde_json::from_str(input).expect("Unable to parse JSON");

  let mut _cells = parsed.get_mut("cells").unwrap();
  // let x: i32 = _cells;

  if let Value::Array(ref mut cells) = _cells {
    for cell in cells {
      if let Some(outputs) = cell.get_mut("outputs") {
        *outputs = json!([]);
      }

      if let Some(execution_count) = cell.get_mut("execution_count") {
        *execution_count = Value::Null;
      }
    }
  }

  return serde_json::to_string_pretty(&parsed).unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_strip() {
    let raw = r###"
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
    "###.trim();

    let exp = r###"
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
      "execution_count": null,
      "metadata": {
        "collapsed": false
      },
      "outputs": [],
      "source": [
        "print(a)"
      ]
    }
  ]
}
    "###.trim();

    let act = strip(raw);

    assert_eq!(act, exp);
  }
}
