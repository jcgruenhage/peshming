use crate::config::{Config, Error};
use std::time::{Duration, Instant};
use tokio::timer::{Interval};
use futures::{future::{lazy, Future}, stream::Stream};
use oping::{Ping};
use log::{trace, debug, info, warn, error};
use prometheus::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref PING_HISTOGRAM : HistogramVec = register_histogram_vec!(
        "ping_rtt_milliseconds",
        "The ping round trip time in milliseconds",
        &["target"],
        vec![0.5, 1.0, 5.0, 10.0, 15.0, 20.0, 25.0, 50.0, 75.0, 100.0, 150.0, 200.0, 250.0,
        300.0, 350.0, 400.0, 450.0, 500.0, 550.0, 600.0, 650.0, 700.0, 750.0, 800.0, 900.0,
        1000.0, 1250.0, 1500.0, 1750.0, 2000.0]
    ).unwrap();
}

pub(crate) fn start_pinging_hosts(config: &Config) {
    for (host, interval) in config.hosts.clone() {
        info!("Spawn ping task for {}", host);
        tokio::spawn(
            Interval::new(Instant::now(), Duration::from_millis(interval))
                .for_each(move |_| {
                    let mut ping = Ping::new();
                    ping.set_timeout(2.5);
                    ping.add_host(&host);
                    for response in match ping.send() {
                        Ok(iterator) => iterator,
                        Err(e) => {
                            error!("Something went wrong sending the ping: {:?}", e);
                            return Ok(());
                        }
                    }{
                        if response.dropped > 0 {
                            debug!("No response from host: {}", response.hostname);
                            PING_HISTOGRAM
                                .with_label_values(&[&host])
                                .observe(2500.0)
                        } else {
                            debug!("Response from host {} (address {}): latency {} ms",
                                     response.hostname, response.address, response.latency_ms);
                            trace!("    all details: {:?}", response);
                            PING_HISTOGRAM
                                .with_label_values(&[&host])
                                .observe(response.latency_ms);
                        }
                    }
                    Ok(())
                }).map_err(|_| ())
        );
    }
}