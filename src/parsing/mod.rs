use std::fs;

use crate::errors::ParseError;

pub fn load_file(file: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(file)?;
    Ok(parse_raw(&raw.trim_matches('\n'))?)
}

fn parse_raw(raw: &str) -> Result<Vec<i32>, ParseError> {
    let mut input: Vec<i32> = Vec::new();

    for int in raw.split(',') {
        let n: i32 = match int.parse() {
            Ok(n) => n,
            Err(e) => return Err(ParseError::new(e.to_string(), int.to_string()))
        };
        input.push(n);
    }

    Ok(input)
}