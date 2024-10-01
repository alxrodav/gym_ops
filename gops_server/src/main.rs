use axum::{http::HeaderValue, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/welcome", get(welcome)).layer(
        CorsLayer::new()
            .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            .allow_methods(Any),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Welcome {
    message: String,
}

async fn welcome() -> impl IntoResponse {
    let welcome = Welcome {
        message: "Hello Mom".to_owned(),
    };
    Json(welcome)
}
