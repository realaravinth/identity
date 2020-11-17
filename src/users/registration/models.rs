/*
* Copyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use super::payload::RegisterCreds;

#[derive(Debug, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub username: String,
    pub email_id: Option<String>,
    pub password: String,
}

impl From<RegisterCreds> for User {
    fn from(creds: RegisterCreds) -> Self {
        User {
            username: creds.username,
            email_id: creds.email_id,
            password: creds.password,
        }
    }
}

//impl User{
//    pub fn insert() -> ServiceResult<User> {
//    }
//}
