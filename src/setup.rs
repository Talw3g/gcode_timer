use super::errors::*;
use std::io::Read;
use std::str;
use std::env;
use std::path::{PathBuf,Path};
use std::fs::File;
use toml;
use clap::{Arg, App};



#[derive(Debug, Deserialize)]
pub struct Config {
    pub cnc: Cnc,
}

#[derive(Debug, Deserialize)]
pub struct Cnc {
    pub speed_x: f32,
    pub speed_y: f32,
    pub speed_z: f32,
}


pub fn get_config() -> Result<(PathBuf, Config, bool,(u8, Option<PathBuf>))> {
    let (input, config_path, list_result, logconf) = get_args()
        .chain_err(|| "Error parsing arguments")?;
    let config = parse_config(config_path)
        .chain_err(|| "Error parsing config file")?;
    Ok((input, config, list_result, logconf))
}


fn parse_config(path: PathBuf) -> Result<(Config)> {

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

fn get_args() -> Result<(PathBuf, PathBuf, bool,(u8, Option<PathBuf>))> {
    let matches = App::new("Gcode_timer")
        .version("0.1.0")
        .author("Thibault M. <tmarion90@gmail.com>")
        .about("A duration estimator for gcode files")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("Displays results tool by tool"))
        .arg(Arg::with_name("v")
            .short("v")
            .conflicts_with("quiet")
            .multiple(true)
            .help("Sets the level of verbosity")
            .long_help("Default is Warn level (-vv).\n\
                If absolutely no output is required, use --quiet"))
        .arg(Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .conflicts_with("v")
            .help("Silences log output (overrides verbosity parameter)"))
        .arg(Arg::with_name("logfile")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Sets log file (maximum verbosity)")
            .takes_value(true))
        .get_matches();

    let input = match matches.value_of("INPUT") {
        Some(s) => {
            Path::new(s).canonicalize()
                .chain_err(|| "Error canonicalizing path")?
        },
        None => bail!("No input file specified"),
    };


    let mut verbose = match matches.occurrences_of("v") as u8 {
        0 => 2,
        x => x,
    };
    if matches.is_present("quiet") {
        verbose = 0;
    }

    let config_path = match matches.value_of("config") {
        Some(s) => {
            Path::new(s).canonicalize()
                .chain_err(|| "Error canonicalizing path")?
        },
        None => {
            let home = match env::home_dir() {
                Some(h) => h,
                None => bail!("Could not get home directory"),
            };
            home.join(Path::new(".gcode_timer/config.toml"))
                .canonicalize()
                .chain_err(|| "Error canonicalizing path")?
        },
    };

    let logfile = match matches.value_of("logfile") {
        Some(s) => Some(PathBuf::from(s)),
        None => None,
    };

    Ok((input, config_path, matches.is_present("list"), (verbose, logfile)))
}
