use log::{error, warn, info, debug, trace};
use futures::future::lazy;

mod config;
mod metrics;
mod ping;
use crate::config::{Config, read_config, setup_clap, setup_fern};
use crate::metrics::start_serving_metrics;
use crate::ping::start_pinging_hosts;

fn main() {
    let clap = setup_clap();
    setup_fern(clap.occurrences_of("v"));
    let config = match read_config(clap.value_of("config").unwrap()) {
        Ok(config) => config,
        Err(_) => {
            error!("Couldn't read config file!");
            return;
        }
    };

    tokio::run(lazy(move || {
        start_serving_metrics(&config);
        start_pinging_hosts(&config);
        Ok(())
    }));
}
