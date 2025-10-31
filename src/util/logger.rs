#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Verbosity {
    Off, // never log
    Error, // only log errors
    On, // always log
}

#[derive(Debug)]
pub struct Logger {
    verbosity: Verbosity
}

impl Logger {
    pub fn from(verbosity: &Verbosity) -> Logger {
        Logger { verbosity: verbosity.clone() }
    }

    pub fn log(&self, msg: &str) {
        if matches!(self.verbosity, Verbosity::On) {
            println!("{}", msg);
        }
    }
    pub fn err(&self, msg: String) {
        if matches!(self.verbosity, Verbosity::Off) {
            eprintln!("{}", msg);
        }
    }
}