use crate::error::error::{Error, ErrorType};
use crate::model::expense_type::ExpenseType;
use sqlx::postgres::PgRow;
use sqlx::{query, Pool, Postgres, Result, Row};

const SQL_FIND_BY_ID: &str = "SELECT * FROM expense_type WHERE id = $1";
const SQL_FIND_ALL: &str = "SELECT * FROM expense_type ORDER BY title ASC";

pub async fn find_all(db_pool: Pool<Postgres>) -> Result<Vec<ExpenseType>, Error> {
    let rows =
        query(SQL_FIND_ALL)
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

// pub async fn create(expense_type: &ExpenseType) -> Result<ExpenseType, Response> {
//     let sql = format!(
//         "INSERT INTO {} (first_name, last_name, city, country, phone)
//         VALUES ($1, $2, $3, $4, $5)
//         RETURNING *",
//         self.get_table_name()
//     );

//     let entity = sqlx::query_as::<Postgres, Customer>(sql.as_str())
//         .bind(customer.first_name.as_str())
//         .bind(customer.last_name.as_str())
//         .bind(customer.city.as_str())
//         .bind(customer.country.as_str())
//         .bind(customer.phone.as_str())
//         .fetch_one(&self.pool.pool)
//         .await
//         .map_err(|error: sqlx::Error| SqlError::new(error.to_string()))?;

//     Ok(entity)
// }

impl Into<ExpenseType> for PgRow {
    fn into(self) -> ExpenseType {
        let id: i32 = self.try_get::<i32, _>("id").unwrap_or_default();
        let title: String = self.try_get::<String, _>("title").unwrap_or_default();
        let description: String = self.try_get::<String, _>("description").unwrap_or_default();
        ExpenseType {
            id,
            title,
            description,
        }
    }
}
