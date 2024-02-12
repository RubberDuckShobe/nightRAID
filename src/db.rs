#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub login_token: String,
    pub balance: i32,
    pub machine_address: String,
    pub last_login: chrono::NaiveDateTime,
}
