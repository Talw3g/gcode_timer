#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

extern crate simplelog;
extern crate toml;
extern crate clap;
extern crate chrono;
extern crate read_lines;

mod errors;
mod setup;
pub mod lineparser;
pub mod objects_def;
mod gcode_lexer;
mod calculator;
mod math_tools;
mod output;

use std::fs::File;
use log::Level;
use chrono::prelude::*;

use setup::{Cnc,get_config};
use read_lines::read_line::LineReader;
use lineparser::parse_line;
use errors::*;
use objects_def::{Machine,Status,Tool};
use output::*;


fn main() {

    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();

        let mut msg = String::from(format!("{}",e));

        for e in e.iter().skip(1) {
            msg.push_str(format!("\n********** Caused by: {}", e).as_str());
        }

        if let Some(backtrace) = e.backtrace() {
            msg.push_str(format!("\n********** Backtrace: {:?}", backtrace).as_str());
        }

        if log_enabled!(Level::Error) {
            error!("{}", msg);
        }
        else {
            writeln!(stderr, "error: {}", msg).expect("Error writing to stderr");
        }
        ::std::process::exit(1);
    }
}


fn run() -> Result<()> {
    let start = Utc::now();

    let (input, config, list_result, logconf) = get_config()
        .chain_err(|| "Could not get configuration")?;
    logger_init(logconf)
        .chain_err(|| "Error initializing logger(s)")?;

    if log_enabled!(Level::Info) {
        info!("Analizing {}", input.to_string_lossy());
    }

    let file = File::open(input)
        .chain_err(|| "Error opening file")?;

    let line_reader = LineReader::new(file)
        .chain_err(|| "Error creating LineReader")?;

    let mut machine = Machine::new(config.cnc);
    let mut warnlog = Warnlog::new();
    let mut tools_list: Vec<Tool> = Vec::new();

    let mut tool = Tool::new(None);

    let mut num_of_line: usize = 0;
    for line in line_reader {
        let line = line
            .chain_err(|| "Error reading line")?;

        if log_enabled!(Level::Info) {num_of_line = num_of_line + 1;}

        if log_enabled!(Level::Trace) {trace!("line: {}",line);}

        let parsed = match parse_line(line)
            .chain_err(|| "Error parsing line")? {
            Some(p) => {
                debug!("Parsed line: {:?}", p);
                p
            },
            None => continue,
        };
        let (modgroup, &tool_number) = machine.line_depacker(parsed)
            .chain_err(|| "Error depacking line")?;
        if log_enabled!(Level::Trace) {trace!("Modgroup: {:?}", modgroup);}

        let (t,d) = modgroup.get_stats(&mut warnlog)
            .chain_err(|| "Error computing stats in modal group")?;

        if tool_number != tool.tool_number {
            debug!("Changing tool number to {:?}", tool_number);
            tools_list.push(tool.clone());
            tool.reset(tool_number);
        }

        tool.duration = tool.duration + t;
        tool.distance = tool.distance + d;

        if let Status::EOP = machine.status {
            tools_list.push(tool);
            warnlog.print_messages();

            let messages = get_tool_messages(tools_list, list_result);
            println!("{}", messages);

            if log_enabled!(Level::Info) {
                let prog_duration: f64 = Utc::now().signed_duration_since(start)
                    .num_microseconds().unwrap() as f64 / 1000.;
                info!("Reached EOP");
                info!("{} lines processed in {:.*}ms", num_of_line, 2, prog_duration);
            }
            ::std::process::exit(0)
        }

        if log_enabled!(Level::Trace) {trace!("End of line\n\n");}
    }

    bail!("Reached EOF without End Of Programm");
}

