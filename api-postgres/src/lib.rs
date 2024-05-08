mod config;
mod database;
mod utils;
mod v1 {
    pub mod routes;
}
mod handlers {
    pub mod health_handlers;
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

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use config::config;
use database::create_pool;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use v1::routes::create_router;

pub struct AppState {}

pub async fn run() {
    let config = config().await;
    let address = format!("{}:{}", config.server_host(), config.server_port());
    let addr: SocketAddr = address
        .parse::<SocketAddr>()
        .expect("Dirección IP inválida");

    let database_url = config.db_url();
    let pool = create_pool(database_url, config.db_max_pool(), config.db_min_pool()).await;

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {})).layer(cors);
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("🚀 Server started successfully on http//:{}", addr);
    axum::serve(listener, app).await.unwrap();
}
