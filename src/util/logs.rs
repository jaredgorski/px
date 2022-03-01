use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    Handle,
};

pub enum Patterns {
    Console,
    Debug,
    File,
}

impl Patterns {
    pub fn as_str(&self) -> &'static str {
        match self {
            Patterns::Console => "[{h({T})}] {m}{n}",
            Patterns::Debug => "{d} {l} {f}, line {L}: [{T} \\({I}\\)] {m}{n}",
            Patterns::File => "{d} | {l} | {T} \\({I}\\) > {m}{n}",
        }
    }
}

pub struct Logs {
    pub handle: Handle,
}

impl Logs {
    pub fn init(level: LevelFilter, verbose: bool) -> Self {
        let handle = log4rs::init_config(Self::get_config(level, verbose)).unwrap();

        Self { handle }
    }

    fn get_config(level: LevelFilter, verbose: bool) -> Config {
        let pattern = match level {
            LevelFilter::Debug => Patterns::Debug.as_str(),
            _ => {
                if verbose {
                    Patterns::File.as_str()
                } else {
                    Patterns::Console.as_str()
                }
            }
        };

        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .build();

        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(level))
            .unwrap()
    }
}