#![allow(unused)]
#![feature(io)]

#[macro_use]
extern crate error_chain;
extern crate walkdir;
extern crate read_lines;

mod other_error {
    error_chain!{}
}

error_chain!{

    links {
        Another(other_error::Error, other_error::ErrorKind) #[cfg(unix)];
    }

}


use std::fs::File;
use std::path::PathBuf;
use read_lines::read_line::LineReader;
//use std::fmt;


fn main() {

    if let Err(ref e) = run() {

        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);
        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

#[derive(Debug)]
enum Codes {
    G(u8),
    X(f32),
    Y(f32),
    Z(f32),
    I(f32),
    J(f32),
}

fn create_code(code: char, acc: &String) -> Option<Codes> {
    match code {
        'G' => {
            let value = acc.trim().parse().unwrap();
            return Some(Codes::G(value))
        },
        'X' => {
            let value = acc.trim().parse().unwrap();
            return Some(Codes::X(value))
        },
        'Y' => {
            let value = acc.trim().parse().unwrap();
            return Some(Codes::Y(value))
        },
        'Z' => {
            let value = acc.trim().parse().unwrap();
            return Some(Codes::Z(value))
        },
        _ => return None,
    }
}

fn parse_line(line: String) -> Result<Option<Vec<Codes>>> {
    let mut line_codes: Vec<Codes> = Vec::new();
    let mut acc = String::new();
    let mut current_letter = None;

    if line.starts_with("(") {
        return Ok(None)
    }

    for item in line.chars() {
        if item.is_alphabetic() {
            if let Some(letter) = current_letter {
                if acc.is_empty() {
                    bail!("Syntax error: two consecutive letters");
                }
                if let Some(code) = create_code(letter, &acc) {
                    line_codes.push(code);
                }
            }
            current_letter = Some(item.to_ascii_uppercase());
            acc.clear();
        }
        else {
            acc.push(item);
        }
    }

    if let Some(letter) = current_letter {
        if acc.is_empty() {
            bail!("Syntax error: no value after letter");
        }
        if let Some(code) = create_code(letter, &acc) {
            line_codes.push(code);
        }
    }

    if line_codes.is_empty() {
        return Ok(None)
    }
    else {
        Ok(Some(line_codes))
    }
}

fn run() -> Result<()> {
    let file = File::open(PathBuf::from("/home/thibault/shared/test.ngc"))
        .chain_err(|| "Error opening file")?;

    let mut line_reader = LineReader::new(file)
        .chain_err(|| "Error creating LineReader")?;

    for line in line_reader {
        let line = line
            .chain_err(|| "Error reading line")?;
        if let Some(splitted) = parse_line(line)
            .chain_err(|| "Error parsing line")? {
            println!("splitted: {:?}", splitted);
        }
    }
    println!("Reached EOF");
    Ok(())
}
