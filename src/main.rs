use axum::{routing::get, Router, serve};
use hyper::server;
use sqlx::Pool;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;
use std::env;
use tracing_subscriber;

mod config;
mod db;
mod auth;
mod models;
mod routes;


#[tokio::main]
async fn main(){
    dotenv().ok();

    tracing_subscriber::fmt()
    .with_env_filter("info")
    .init();

    let pool = db::connect_pg().await.expect("DB CONNECTION FAILED !!");
    let app = Router::new()
    .merge(routes::create_auth_routes(pool.clone()))
    .merge(routes::create_ticket_routes(pool.clone()))
    .route("/", get(|| async { "Running" }))
    .layer(CorsLayer::new().allow_origin(Any));

    // Read port from env
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Server running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}