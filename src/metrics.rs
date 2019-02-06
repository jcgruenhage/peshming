use crate::config::{Config};
use lazy_static::lazy_static;
use hyper::{Server, Response, Body, header::CONTENT_TYPE, service::service_fn_ok};
use prometheus::{TextEncoder, Counter, Gauge, HistogramVec};
use prometheus::*;

use futures::future::Future;

lazy_static! {
    static ref HTTP_COUNTER: Counter = register_counter!(opts!(
        "http_requests_total",
        "Total number of HTTP requests made.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_BODY_GAUGE: Gauge = register_gauge!(opts!(
        "http_response_size_bytes",
        "The HTTP response sizes in bytes.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["handler"]
    )
    .unwrap();
}

pub(crate) fn start_serving_metrics(config: &Config) {
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
    println!("Listening on {}", &config.listener);
    let server = Server::bind(&config.listener)
        .serve(serve_metrics)
        .map_err(|err| eprintln!("server error: {}", err));
    tokio::spawn(server);
}