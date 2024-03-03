use crate::service::expense_type_service;
use axum::{
    routing::{delete, get, post, put},
    Extension, Router,
};
use sqlx::postgres::Postgres;
use sqlx::Pool;

pub fn create_routes(db_pool: Pool<Postgres>) -> Router<()> {
    Router::new()
        .route("/expense_type", get(expense_type_service::find_all_))
        .route("/expense_type/:id", get(expense_type_service::find_by_id_))
        .route("/expense_type", post(expense_type_service::create_))
        .route("/expense_type/:id", put(expense_type_service::update_))
        .route("/expense_type/:id", delete(expense_type_service::delete_))
        .layer(Extension(db_pool))
}
