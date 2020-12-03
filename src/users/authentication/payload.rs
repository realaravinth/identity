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

use argon2::verify_encoded;
use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::errors::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct LoginCreds {
    pub username: String,
    pub password: String,
}

impl LoginCreds {
    pub async fn verify(&self, db_pool: &Pool) -> ServiceResult<()> {
        let statement = format!(
            " SELECT password FROM users WHERE username = '{}'",
            &self.username
        );
        let client: Client = db_pool.get().await?;
        let command = client.prepare(&statement).await.unwrap();

        let mut rows = client.query(&command, &[]).await.unwrap();
        if let Some(val) = rows.pop() {
            if val.is_empty() {
                Err(ServiceError::UserDoesntExist)
            } else {
                let hash: &str = val.get(0);
                if verify_encoded(hash, self.password.as_bytes())? {
                    Ok(())
                } else {
                    Err(ServiceError::AuthorizationRequired)
                }
            }
        } else {
            Err(ServiceError::UserDoesntExist)
        }
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }
}
