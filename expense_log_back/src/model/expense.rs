#[derive(Clone, Debug)]
pub struct Model {
    pub id: i32,
    pub expense_date: Date,
    pub expense_type_id: i32,
    pub amount: Decimal,
    pub description: String,
    pub app_user_id: i32,
}
