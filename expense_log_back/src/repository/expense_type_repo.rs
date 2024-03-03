use crate::error::error::{Error, ErrorType};
use crate::model::expense_type::ExpenseType;
use sqlx::postgres::PgRow;
use sqlx::{query, Pool, Postgres, Result, Row};

const SQL_FIND_BY_ID: &str = "SELECT * FROM expense_type WHERE id = $1";
const SQL_FIND_ALL: &str = "SELECT * FROM expense_type";
const SQL_CREATE: &str =
    "INSERT INTO expense_type (title, description) VALUES ($1, $2) RETURNING *";
const SQL_UPDATE: &str =
    "UPDATE expense_type SET title = $1, description = $2 WHERE id = $3 RETURNING *";
const SQL_DELETE: &str = "DELETE FROM expense_type WHERE id = $1";

pub async fn find_all(
    db_pool: Pool<Postgres>,
    order_by: Option<String>,
    sort_order: Option<String>,
) -> Result<Vec<ExpenseType>, Error> {
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

    let expense_types: Vec<ExpenseType> = rows.into_iter().map(|row| row.into()).collect();
    Ok(expense_types)
}

pub async fn find_by_id(db_pool: Pool<Postgres>, id: i32) -> Result<ExpenseType, Error> {
    let row = query(SQL_FIND_BY_ID)
        .bind(id)
        .fetch_one(&db_pool)
        .await
        .map_err(|_| {
            Error::new(
                ErrorType::EntityNotFound,
                format!("ExpenseType with id {} not found", id),
            )
        })?;

    let expense_type = row.into();
    Ok(expense_type)
}

pub async fn create(
    db_pool: Pool<Postgres>,
    expense_type: &ExpenseType,
) -> Result<ExpenseType, Error> {
    let row = sqlx::query(SQL_CREATE)
        .bind(expense_type.title.as_str())
        .bind(expense_type.description.as_deref().unwrap_or_default())
        .fetch_one(&db_pool)
        .await
        .map_err(|error: sqlx::error::Error| {
            Error::new(ErrorType::InternalServerError, error.to_string())
        })?;

    let expense_type = row.into();
    Ok(expense_type)
}

pub async fn update(
    db_pool: Pool<Postgres>,
    id: i32,
    expense_type: &ExpenseType,
) -> Result<ExpenseType, Error> {
    find_by_id(db_pool.clone(), id).await?;

    let row = sqlx::query(SQL_UPDATE)
        .bind(expense_type.title.as_str())
        .bind(expense_type.description.as_deref().unwrap_or_default())
        .bind(id)
        .fetch_one(&db_pool)
        .await
        .map_err(|error| Error::new(ErrorType::InternalServerError, error.to_string()))?;

    let expense_type = row.into();
    Ok(expense_type)
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

impl Into<ExpenseType> for PgRow {
    fn into(self) -> ExpenseType {
        let id: i32 = self.try_get::<i32, _>("id").unwrap_or_default();
        let title: String = self.try_get::<String, _>("title").unwrap_or_default();
        let description: Option<String> =
            Some(self.try_get::<String, _>("description").unwrap_or_default());
        ExpenseType {
            id,
            title,
            description,
        }
    }
}
