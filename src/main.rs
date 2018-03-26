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




fn run() -> Result<()> {
    let file = File::open(PathBuf::from("filepath"))
        .chain_err(|| "Error opening file")?;

    let mut line_r = LineReader::new(file)
        .chain_err(|| "Error creating LineReader")?;

    for line in line_r {
        let line = line
            .chain_err(|| "Error reading line")?;
        println!("{}",line);
    }
    println!("Reached EOF");
    Ok(())
}
