mod handlers;
mod models;
mod persistence;

use axum::{routing::post, Extension, Router};
use dotenvy::dotenv;
use handlers::log::{create_log, get_logs};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = persistence::init_db(&database_url).await;

    let app = Router::new()
        .route("/log", post(create_log).get(get_logs))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
