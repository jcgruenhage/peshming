extern crate serde_derive;
extern crate toml;
extern crate hyper;
extern crate lazy_static;
extern crate prometheus;
extern crate futures;
extern crate tokio;
extern crate env_logger;

use hyper::header::CONTENT_TYPE;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

use lazy_static::lazy_static;

use prometheus::{Counter, Encoder, Gauge, HistogramVec, TextEncoder};
use prometheus::*;

use futures::future::lazy;

//TODO: Replace with useful metrics, these are beyond useless
lazy_static! {
    static ref HTTP_COUNTER: Counter = register_counter!(opts!(
        "example_http_requests_total",
        "Total number of HTTP requests made.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_BODY_GAUGE: Gauge = register_gauge!(opts!(
        "example_http_response_size_bytes",
        "The HTTP response sizes in bytes.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "example_http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    )
    .unwrap();
}

fn start_serving_metrics() {
    let serve_metrics = || {
        service_fn_ok(|_req| {
            HTTP_COUNTER.inc();
            let timer = HTTP_REQ_HISTOGRAM.with_label_values(&["all"]).start_timer();
            let metric_families = prometheus::gather();
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            encoder.encode(&metric_families, &mut buffer).unwrap();
            HTTP_BODY_GAUGE.set(buffer.len() as f64);
            let mut res = Response::new(Body::from(buffer));
            res.headers_mut().insert(CONTENT_TYPE, encoder.format_type().parse().unwrap());
            timer.observe_duration();
            res
        })
    };
    println!("listening addr 127.0.0.1:9898");
    let server = Server::bind(&([127, 0, 0, 1], 9898).into())
        .serve(serve_metrics)
        .map_err(|err| eprintln!("server error: {}", err));
    tokio::spawn(server);
}

fn start_pinging_hosts() {
    tokio::spawn(lazy(|| {
        //TODO: Implement the pinging, based on oping and the example over at
        //      https://tokio.rs/docs/futures/spawning/
        Ok(())
    }));
}
    //TODO: Clean this shameful mess up!
    //TODO: Do config reading, cli args, logging etc


fn main() {
    env_logger::init();
    tokio::run(lazy(|| {
        start_serving_metrics();
        start_pinging_hosts();
        Ok(())
    }));
}
