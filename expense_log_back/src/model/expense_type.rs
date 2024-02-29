use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpenseType {
    pub id: i32,
    pub title: String,
    pub description: String,
}
