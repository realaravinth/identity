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

mod utils;

mod authentication;
mod handlers;
mod registration;
mod routes;

use authentication::routes as authentication_routes;
use registration::routes as registration_routes;
pub use routes::routes;
pub use utils::{beep, PROFAINITY};
pub use utils::{filter, USERNAME_CASE_MAPPED};
pub use utils::{forbidden, BLACKLIST};
