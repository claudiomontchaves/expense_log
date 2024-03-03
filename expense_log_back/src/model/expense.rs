use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Expense {
    pub id: i32,
    pub expense_date: String,
    pub expense_type_id: i32,
    pub amount: f32,
    pub description: Option<String>,
    pub app_user_id: i32,
}
