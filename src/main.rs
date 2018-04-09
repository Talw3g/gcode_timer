#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate toml;
extern crate read_lines;

mod errors;
pub mod lineparser;
pub mod objects_def;
mod gcode_lexer;
mod calculator;
mod math_tools;
mod warnings;

use std::fs::File;
use std::path::{PathBuf,Path};
use std::io::Read;
use std::str;
use std::env;

use read_lines::read_line::LineReader;
use lineparser::parse_line;
use errors::*;
use objects_def::{Machine,Status,Tool,get_tool_messages};
use warnings::Warnlog;

#[derive(Debug, Deserialize)]
struct Config {
    cnc: Cnc,
}

#[derive(Debug, Deserialize)]
pub struct Cnc {
    pub speed_x: f32,
    pub speed_y: f32,
    pub speed_z: f32,
}


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
    let config = get_config()
        .chain_err(|| "Error parsing config file")?;

    let file = File::open(PathBuf::from("/home/thibault/shared/manual.ngc"))
        .chain_err(|| "Error opening file")?;

    let line_reader = LineReader::new(file)
        .chain_err(|| "Error creating LineReader")?;

    let mut machine = Machine::new(config.cnc);
    let mut warnlog = Warnlog::new();
    let mut tools_list: Vec<Tool> = Vec::new();

    let mut tool = Tool::new(None);

    for line in line_reader {
        let line = line
            .chain_err(|| "Error reading line")?;
        let parsed = match parse_line(line)
            .chain_err(|| "Error parsing line")? {
            Some(p) => p,
            None => continue,
        };
        let (modgroup, &tool_number) = machine.line_depacker(parsed)
            .chain_err(|| "Error depacking line")?;

        let (t,d) = modgroup.get_stats(&mut warnlog)
            .chain_err(|| "Error computing stats in modal group")?;

        if tool_number != tool.tool_number {
            tools_list.push(tool.clone());
            tool.reset(tool_number);
        }
        tool.duration = tool.duration + t;
        tool.distance = tool.distance + d;
        if let Status::EOP = machine.status {
            tools_list.push(tool);
            warnlog.print_messages();
            let messages = get_tool_messages(tools_list);
            println!("{}", messages);
            println!("Reached EOP");
            ::std::process::exit(0)
        }
    }
    bail!("Reached EOF without End Of Programm");
}

fn get_config() -> Result<Config> {
    let home = match env::home_dir() {
        Some(h) => h,
        None => bail!("Could not get home directory"),
    };
    let config_path = Path::new(".gcode_timer/config.toml");
    let path = home.join(config_path).canonicalize()
        .chain_err(|| "Error canonicalizing path")?;

    let mut file = File::open(path)
        .chain_err(|| "Error opening config file")?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .chain_err(|| "Error reading config file")?;

    let filestr = str::from_utf8(&buffer)
        .chain_err(|| "Error converting buffer to utf8 str")?;
    let config: Config = toml::from_str(filestr)
        .chain_err(|| "Error parsing Config from config file")?;

    Ok(config)

}
