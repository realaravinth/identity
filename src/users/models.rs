use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct User {
    pub username: String,
    pub user_uuid: u32,
    pub hash: String,
    pub email: String,
    pub role: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Creds {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertableCreds {
    pub normalised_username: String,
    pub hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewCreds {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}
