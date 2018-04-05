//use super::errors::*;
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
                                            duration results may be imprecise.\n\
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
            println!("WARNING:\n{}\n\n", mess);
        }
    }
}
