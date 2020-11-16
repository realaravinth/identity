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

use actix_web::web;

use super::handlers::{sign_in, sign_out};
use super::registration_routes;

#[cfg(not(tarpaulin_include))]
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/signin").route(web::post().to(sign_in)));
    cfg.service(web::resource("/api/signout").route(web::post().to(sign_out)));
    registration_routes(cfg);
}
