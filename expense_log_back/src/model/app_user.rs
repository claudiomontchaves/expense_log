#[derive(Clone, Debug)]
pub struct Model {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub admin_user: bool,
}
