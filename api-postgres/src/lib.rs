mod v1 {
    pub mod routes;
}
mod handlers {
    pub mod location_handlers;
}
mod services {
    pub mod location_services;
}
mod schemas {
    pub mod location_schemas;
}
mod models {
    pub mod location_model;
}

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    Router,
};
use dotenv::dotenv;
use std::env::{self};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use v1::routes::setup_routes;

pub async fn run() {
    dotenv().ok();

    let port = env::var("APP_PORT").expect("APP_PORT must be set");
    let address = format!("0.0.0.0:{}", port);
    let addr = address
        .parse::<SocketAddr>()
        .expect("Dirección IP inválida");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let api_routes = setup_routes();
    let app = Router::new().nest("/api/v1", api_routes).layer(cors);

    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
