#[macro_use]
extern crate error_chain;
extern crate read_lines;

mod errors;
pub mod lineparser;
pub mod gcodes_def;
mod gcode_lexer;

use std::fs::File;
use std::path::PathBuf;

use read_lines::read_line::LineReader;
use lineparser::parse_line;
use errors::*;
//use gcode_lexer::line_depacker;
use gcodes_def::{Machine,GCode};
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


fn run() -> Result<()> {
    let file = File::open(PathBuf::from("/home/thibault/shared/test.ngc"))
        .chain_err(|| "Error opening file")?;

    let line_reader = LineReader::new(file)
        .chain_err(|| "Error creating LineReader")?;

    let mut machine = Machine::new(550.0);

    for line in line_reader {
        let line = line
            .chain_err(|| "Error reading line")?;
        let parsed = match parse_line(line)
            .chain_err(|| "Error parsing line")? {
            Some(p) => p,
            None => continue,
        };
        let modgroup = match machine.line_depacker(parsed)
            .chain_err(|| "Error depacking line")? {
            Some(m) => m,
            None => continue,
        };
    }
    println!("Machine: {:?}", machine);
    println!("Reached EOF");
    Ok(())
}

