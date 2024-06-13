mod handlers;
mod models;
mod persistence;

use axum::{routing::post, Extension, Router};
use dotenvy::dotenv;
use handlers::temperature::{create_temperature, get_temperature, update_temperature};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = persistence::init_db(&database_url).await;

    let app = Router::new()
        .route(
            "/temperature",
            post(create_temperature)
                .get(get_temperature)
                .put(update_temperature),
        )
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
