use simplelog::*;
use std::fs::File;
use std::path::PathBuf;

use super::errors::*;
use objects_def::Tool;

pub fn logger_init(logconf: (u8, Option<PathBuf>)) -> Result<()> {
    let (verbose, logfile) = logconf;
    let level = match verbose {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 | _ => LevelFilter::Trace,
    };

    if let Some(f) = logfile {
        CombinedLogger::init(vec![
            match TermLogger::new(level, Config::default()) {
                Some(t) => t,
                None => bail!("Error creating TermLogger"),
            },
            WriteLogger::new(LevelFilter::Trace, Config::default(), File::create(f)
                .chain_err(|| "Error creating file")?),
        ]).chain_err(|| "Error creating CombinedLogger")?;
    }
    else {
        TermLogger::init(level, Config::default())
            .chain_err(|| "Error creating TermLogger")?;
    }
    Ok(())
}



#[derive(PartialEq)]
pub enum WarnType {
    TooFast,
}

pub struct Warnlog {
    messages: Vec<String>,
    logged_types: Vec<WarnType>,
}

impl Warnlog {
    pub fn new() -> Warnlog {
        Warnlog {
            messages: Vec::new(),
            logged_types: Vec::new(),
        }
    }

    pub fn warn(&mut self, t: WarnType) {
        match t {
            WarnType::TooFast => {
                let message = String::from("Feed speed is higher than set machine's capabilities: \
                                            duration results may be imprecise. \
                                            Moreover, it will probably damage the CNC.");
                self.store_messages(message,t);
            },
        }
    }

    fn store_messages(&mut self, message: String, t: WarnType) {
        if !self.logged_types.contains(&t) {
            self.logged_types.push(t);
            self.messages.push(message);
        }
    }

    pub fn print_messages(&self) {
        for mess in self.messages.iter() {
            warn!("{}\n\n", mess);
        }
    }
}

pub fn get_tool_messages(tools_list: Vec<Tool>, list_result: bool) -> String {
    let mut message = String::new();
    let mut total_dist = 0.;
    let mut total_dura = 0.;

    for item in tools_list.iter() {
        if list_result {
            match item.tool_number {
                Some(u) => message.push_str(format!("Tool {}:\n", u).as_str()),
                None if item.distance != 0. => message.push_str("No tool:\n"),
                None => continue,
            }
            message.push_str(format!("  Duration: {}\n", time_format(item.duration)).as_str());
            message.push_str(format!("  Distance: {:.*}mm\n\n", 1, item.distance).as_str());
        }
        total_dist = total_dist + item.distance;
        total_dura = total_dura + item.duration;
    }

    message.push_str(format!("Total duration: {}\n", time_format(total_dura)).as_str());
    message.push_str(format!("Total distance: {:.*}mm\n", 1, total_dura).as_str());
    message
}

fn time_format(mut t: f32) -> String {
    let mut out = String::new();

    let h = (t/3600.).floor();
    if h != 0. {
        out.push_str(format!("{}h",h).as_str());
    }

    t = t - h*3600.;

    let m = (t/60.).floor();
    if m != 0. {
        out.push_str(format!("{}m",m).as_str());
    }

    t = t - m*60.;

    if t != 0. {
        out.push_str(format!("{:.*}s",0,t).as_str());
    }
    else {
        out.push_str("< 1s");
    }

    out
}
