use crate::error::error::{Error, ErrorType};
use crate::model::app_user::AppUser;
use sqlx::postgres::PgRow;
use sqlx::{query, Pool, Postgres, Result, Row};

const SQL_FIND_BY_ID: &str = "SELECT * FROM app_user WHERE id = $1";
const SQL_FIND_ALL: &str = "SELECT * FROM app_user";
const SQL_CREATE: &str = "INSERT INTO app_user (username, email, password, admin_user) VALUES ($1, $2, $3, $4) RETURNING *";
const SQL_UPDATE: &str =
    "UPDATE app_user SET username = $1, email = $2, password = $3, admin_user = $4 WHERE id = $5 RETURNING *";
const SQL_DELETE: &str = "DELETE FROM app_user WHERE id = $1";

pub async fn find_all(
    db_pool: Pool<Postgres>,
    order_by: Option<String>,
    sort_order: Option<String>,
) -> Result<Vec<AppUser>, Error> {
    let order_by = order_by.unwrap_or_else(|| "id".to_string());
    let sort_order = sort_order.unwrap_or_else(|| "ASC".to_string());

    let sql = format!("{} ORDER BY {} {}", SQL_FIND_ALL, order_by, sort_order);

    let rows = query(sql.as_str())
        .bind(order_by)
        .fetch_all(&db_pool)
        .await
        .map_err(|error: sqlx::error::Error| {
            Error::new(ErrorType::InternalServerError, error.to_string())
        })?;

    let app_users: Vec<AppUser> = rows.into_iter().map(|row| row.into()).collect();
    Ok(app_users)
}

pub async fn find_by_id(db_pool: Pool<Postgres>, id: i32) -> Result<AppUser, Error> {
    let row = query(SQL_FIND_BY_ID)
        .bind(id)
        .fetch_one(&db_pool)
        .await
        .map_err(|_| {
            Error::new(
                ErrorType::EntityNotFound,
                format!("AppUser with id {} not found", id),
            )
        })?;

    let app_user = row.into();
    Ok(app_user)
}

pub async fn create(db_pool: Pool<Postgres>, app_user: &AppUser) -> Result<AppUser, Error> {
    let row = sqlx::query(SQL_CREATE)
        .bind(app_user.username.as_str())
        .bind(app_user.email.as_str())
        .bind(app_user.password.as_str())
        .bind(app_user.admin_user)
        .fetch_one(&db_pool)
        .await
        .map_err(|error: sqlx::error::Error| {
            Error::new(ErrorType::InternalServerError, error.to_string())
        })?;

    let app_user = row.into();
    Ok(app_user)
}

pub async fn update(
    db_pool: Pool<Postgres>,
    id: i32,
    app_user: &AppUser,
) -> Result<AppUser, Error> {
    find_by_id(db_pool.clone(), id).await?;

    let row = sqlx::query(SQL_UPDATE)
        .bind(app_user.username.as_str())
        .bind(app_user.email.as_str())
        .bind(app_user.password.as_str())
        .bind(app_user.admin_user)
        .bind(id)
        .fetch_one(&db_pool)
        .await
        .map_err(|error| Error::new(ErrorType::InternalServerError, error.to_string()))?;

    let app_user = row.into();
    Ok(app_user)
}

pub async fn delete(db_pool: Pool<Postgres>, id: i32) -> Result<(), Error> {
    find_by_id(db_pool.clone(), id).await?;

    let _ = sqlx::query(SQL_DELETE)
        .bind(id)
        .execute(&db_pool)
        .await
        .map_err(|error| Error::new(ErrorType::InternalServerError, error.to_string()))?;

    Ok(())
}

impl Into<AppUser> for PgRow {
    fn into(self) -> AppUser {
        let id: i32 = self.try_get::<i32, _>("id").unwrap_or_default();
        let username: String = self.try_get::<String, _>("username").unwrap_or_default();
        let email: String = self.try_get::<String, _>("email").unwrap_or_default();
        let password: String = self.try_get::<String, _>("password").unwrap_or_default();
        let admin_user: bool = self.try_get::<bool, _>("admin_user").unwrap_or_default();
        AppUser {
            id,
            username,
            email,
            password,
            admin_user,
        }
    }
}
