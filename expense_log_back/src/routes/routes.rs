use axum::{routing::get, Extension, Router};
use sqlx::postgres::Postgres;
use sqlx::Pool;

pub fn create_routes(db_pool: Pool<Postgres>) -> Router<()> {
    Router::new()
        .route("/hello", get("Hello, World!"))
        .layer(Extension(db_pool))
}
