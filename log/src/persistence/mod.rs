use sqlx::postgres::PgPoolOptions;

pub async fn init_db(database_url: &str) -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create pool")
}
