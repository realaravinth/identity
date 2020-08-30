use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub username: String,
    pub user_uuid: u32,
    pub hash: String,
    pub email: String,
    pub role: String,
    pub name: String,
}
