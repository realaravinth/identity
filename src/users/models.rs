use diesel::prelude::*;

#[derive(Insertable)]
pub struct Users {
    pub username: String,
    pub user_uuid: u32,
    pub hash: String,
    pub email: String,
    pub role: String,
    pub name: String,
}
