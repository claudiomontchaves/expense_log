use crate::error::error::Error;
use crate::model::expense_type::ExpenseType;
use crate::repository::expense_type_repo::{find_all, find_by_id};
use axum::response::{IntoResponse, Response};
use axum::{extract::Path, Extension, Json};
use sqlx::{Pool, Postgres};

pub async fn find_all_(
    Extension(db_pool): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<ExpenseType>>, Response> {
    let expense_types: Vec<ExpenseType> = find_all(db_pool)
        .await
        .map_err(|error: Error| error.into_response())?;
    Ok(Json(expense_types))
}

pub async fn find_by_id_(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<ExpenseType>, Response> {
    let expense_type = find_by_id(db_pool, id)
        .await
        .map_err(|error: Error| error.into_response())?;
    Ok(Json(expense_type))
}
