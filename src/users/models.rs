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

use pow_sha256::PoW;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Queryable)]
pub struct User {
    pub username: String,
    pub hash: String,
    pub email: String,
    pub role: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Queryable)]
pub struct InsertableCreds {
    pub username: String,
    pub hash: String,
    pub email: Option<String>,
    pub role: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Creds {
    pub username: String,
    pub password: String,
}

//#[derive(Debug, Deserialize, PartialEq, Serialize)]
//pub struct ProcessCreds {
//    pub normalised_username: String,
//    pub hash: String,
//}

impl InsertableCreds {
    pub fn default(username: String, hash: String) -> Self {
        let name = username.clone();
        let email = None;
        let role = "user".to_string();
        InsertableCreds {
            username,
            hash,
            email,
            role,
            name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insertable_default() {
        let creds = InsertableCreds::default(String::from("yadayada"), String::from("asdasdasd"));
        assert_eq!(creds.username, creds.name);
        assert_eq!(creds.username, "yadayada");
        assert_eq!(creds.role, "user");
    }
}
