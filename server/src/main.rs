extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod colors;
pub mod middleware;

use std::{env, net::SocketAddr, str::FromStr};

use admyre::{
    layout::maven::{make_test_data, render_index},
    maven::metadata::make_example_metadata,
    xml::maven::serialize_maven_metadata,
};

use axum::{
    http::{HeaderName, HeaderValue, Response},
    middleware::from_fn,
    routing::get,
    Router, Server,
};

use tokio::main;

use crate::middleware::logger::logging_middleware;

pub async fn get_index() -> Response<String> {
    let html = render_index("/".to_string(), make_test_data());
    let mut res = Response::new(html);

    res.headers_mut().append(
        HeaderName::from_str("Content-Type").unwrap(),
        HeaderValue::from_str("text/html").unwrap(),
    );

    return res;
}

pub async fn get_metadata() -> Response<String> {
    let metadata = make_example_metadata();
    let data = serialize_maven_metadata(metadata);
    let mut res = Response::new(data);

    res.headers_mut().append(
        HeaderName::from_str("Content-Type").unwrap(),
        HeaderValue::from_str("application/xml").unwrap(),
    );

    return res;
}

#[main]
pub async fn main() {
    env::set_var("RUST_LOG", "info");

    pretty_env_logger::init();

    let app = Router::new()
        .route("/", get(get_index))
        .route("/maven-metadata.xml", get(get_metadata))
        .layer(from_fn(logging_middleware));

    let addr: SocketAddr = "0.0.0.0:4000".parse().unwrap();

    let server = Server::bind(&addr.clone()).serve(app.into_make_service());

    info!(target: "Admyre Server", "Server listening on {}:{}!", addr.ip().to_string(), addr.port());

    server.await.unwrap();
}
