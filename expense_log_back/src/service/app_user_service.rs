use crate::error::error::Error;
use crate::model::app_user::AppUser;
use crate::repository::app_user_repo::{create, delete, find_all, find_by_id, update};
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
) -> Result<Json<Vec<AppUser>>, Response> {
    let app_users: Vec<AppUser> = find_all(db_pool, parameters.order_by, parameters.sort_order)
        .await
        .map_err(|error: Error| error.into_response())?;
    Ok(Json(app_users))
}

pub async fn find_by_id_(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<AppUser>, Response> {
    let app_user = find_by_id(db_pool, id)
        .await
        .map_err(|error: Error| error.into_response())?;
    Ok(Json(app_user))
}

pub async fn create_(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Json(app_user): Json<AppUser>,
) -> Result<(StatusCode, Json<AppUser>), Response> {
    let app_user = create(db_pool, &app_user)
        .await
        .map_err(|error: Error| error.into_response())?;
    Ok((StatusCode::CREATED, Json(app_user)))
}

pub async fn update_(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(app_user): Json<AppUser>,
) -> Result<Json<AppUser>, Response> {
    let app_user = update(db_pool, id, &app_user)
        .await
        .map_err(|error: Error| error.into_response())?;
    Ok(Json(app_user))
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
