mod v1 {
    pub mod routes;
}

use axum::Router;
use dotenv::dotenv;
//use std::env;
use v1::routes::setup_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_routes = setup_routes();
    let app = Router::new().nest("/api/v1", api_routes);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
