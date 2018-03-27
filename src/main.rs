#[macro_use]
extern crate error_chain;
extern crate walkdir;
extern crate read_lines;

mod errors;
//mod other_error {
//    error_chain!{}
//}
//
//error_chain!{
//
//    links {
//        Another(other_error::Error, other_error::ErrorKind) #[cfg(unix)];
//    }
//
//}


use std::fs::File;
use std::path::PathBuf;
use read_lines::read_line::LineReader;
use lineparser::parse_line;
use errors::*;
//use std::fmt;

pub mod lineparser;

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


fn run() -> Result<()> {
    let file = File::open(PathBuf::from("/home/thibault/shared/test.ngc"))
        .chain_err(|| "Error opening file")?;

    let line_reader = LineReader::new(file)
        .chain_err(|| "Error creating LineReader")?;

    for line in line_reader {
        let line = line
            .chain_err(|| "Error reading line")?;
        if let Some(parsed) = parse_line(line)
            .chain_err(|| "Error parsing line")? {
            println!("{:?}", parsed);
        }
    }
    println!("Reached EOF");
    Ok(())
}
