use crate::error::error::Error;
use crate::model::expense_type::ExpenseType;
use crate::repository::expense_type_repo::{create, delete, find_all, find_by_id, update};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
pub struct Parameters {
    pub order_by: Option<String>,
    pub sort_order: Option<String>,
}

pub async fn find_all_(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Query(parameters): Query<Parameters>,
) -> Result<Json<Vec<ExpenseType>>, Response> {
    let expense_types: Vec<ExpenseType> =
        find_all(db_pool, parameters.order_by, parameters.sort_order)
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

pub async fn create_(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Json(expense_type): Json<ExpenseType>,
) -> Result<(StatusCode, Json<ExpenseType>), Response> {
    let expense_type = create(db_pool, &expense_type)
        .await
        .map_err(|error: Error| error.into_response())?;
    Ok((StatusCode::CREATED, Json(expense_type)))
}

pub async fn update_(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(expense_type): Json<ExpenseType>,
) -> Result<Json<ExpenseType>, Response> {
    let expense_type = update(db_pool, id, &expense_type)
        .await
        .map_err(|error: Error| error.into_response())?;
    Ok(Json(expense_type))
}

pub async fn delete_(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<(), Response> {
    delete(db_pool, id)
        .await
        .map_err(|error: Error| error.into_response())?;
    Ok(())
}
