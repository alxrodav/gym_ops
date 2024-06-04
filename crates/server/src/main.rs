mod config;
mod logger;
use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};
use config::Config;
use logger::init_logger;

#[tokio::main]
async fn main() {
    init_logger();

    let config = Config::from_env();
    let app = Router::new().route("/", get(hello_world));

    let address = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    log::info!("server lintening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> Html<String> {
    Html("<h1>Hello World!!!</h1>".into())
}
