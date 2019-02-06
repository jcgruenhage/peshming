use std::collections::HashMap;
use log::{error, warn, info, debug, trace};
use serde::{Serialize, Deserialize};
use clap::{clap_app, crate_name, crate_version, crate_description, crate_authors};

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) listener: std::net::SocketAddr,
    pub(crate) hosts: HashMap<String, u64>,
}

pub(crate) fn setup_clap() -> (clap::ArgMatches<'static>) {
    clap_app!(myapp =>
        (name: crate_name!())
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg config: +required "Set config file")
        (@arg v: -v --verbose ... "Be verbose (you can add this up to 4 times for more logs)")
    ).get_matches()
}

pub(crate) fn setup_fern(level: u64) {
    let level = match level {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace
    };
    match fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply() {
        Err(_) => {
            eprintln!("error setting up logging!");
        }
        _ => info!("logging set up properly"),
    }
}

pub(crate) fn read_config(path: &str) -> Result<Config, Error> {
    let config_file_content = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&config_file_content)?)
}

pub(crate) struct Error {}

impl std::convert::From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error{}
    }
}

impl std::convert::From<toml::de::Error> for Error {
    fn from(_: toml::de::Error) -> Self {
        Error{}
    }
}

impl std::convert::From<oping::PingError> for Error {
    fn from(_: oping::PingError) -> Self {
        Error{}
    }
}
