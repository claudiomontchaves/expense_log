use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub admin_user: bool,
}
