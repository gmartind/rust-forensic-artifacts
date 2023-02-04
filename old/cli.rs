use std::env;

mod controller;

#[derive(PartialEq, Eq)]
enum Args {
    HELP,
    START,
    UNKNOWN,
}

pub struct CLI {
    control: controller::Controller,
}

impl CLI {
    pub fn new() -> Self {
        Self {
            control: controller::Controller::new(),
        }
    }

    fn display_help(&mut self) {
        println!("HELP");
    }

    fn acquisition(&mut self) {
        match self.control.acquire() {
            Ok(_) => println!("Acquisiton worked!"),
            Err(e) => println!("{}", e),
        }
    }

    fn check_options(&mut self, vec: Vec<Args>) -> Result<Vec<Args>, String> {
        if vec.contains(&Args::UNKNOWN) || vec.len() > 2 {
            return Err("Invalid syntax".to_string());
        }

        Ok(vec)
    }

    fn parse_options(&mut self, vec: Vec<Args>) {
        if vec.contains(&Args::HELP) {
            self.display_help();
        }
        if vec.contains(&Args::START) {
            self.acquisition();
        }
    }

    fn get_options(&mut self) -> Option<Vec<Args>> {
        let arguments: Vec<String> = env::args().collect();
        if arguments.len() < 2 {
            return None;
        }
        let mut ret: Vec<Args> = Vec::new();
        let mut i = 1;
        while i < arguments.len() {
            if arguments[i] == "-h".to_string() {
                ret.push(Args::HELP);
            } else if arguments[1] == "-s".to_string() {
                ret.push(Args::START);
            } else {
                ret.push(Args::UNKNOWN);
            }
            i += 1;
        }
        Some(ret)
    }

    pub fn start(&mut self) {
        match self.get_options() {
            Some(vec) => match self.check_options(vec) {
                Ok(vec) => self.parse_options(vec),
                Err(e) => println!("{}", e),
            },
            None => self.acquisition(),
        }
    }
}
