// Copyright (c) 2020 Aravinth T M <realaravinth@batsense.net>.
// See the COPYRIGHT file at the top-level directory of this
// distribution

//This program is free software; you can redistribute it and/or
//modify it under the terms of the GNU General Public License
//as published by the Free Software Foundation; either version 2
//of the License, or (at your option) any later version.

//This program is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.

//You should have received a copy of the GNU General Public License
//along with this program; if not, write to the Free Software
//Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

use pow_sha256::PoW;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Queryable)]
pub struct User {
    pub username: String,
    pub user_uuid: u32,
    pub hash: String,
    pub email: String,
    pub role: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Creds {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct InsertableCreds {
    pub normalised_username: String,
    pub hash: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct NewCreds {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub pow: PoW<Vec<u8>>,
}
