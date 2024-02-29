use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpenseAlert {
    pub id: i32,
    pub day_of_month: i32,
    pub total_value: f32,
}
