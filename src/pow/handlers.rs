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
use actix_session::Session;
use actix_web::{HttpResponse, Responder};

use super::PoWConfig;
use crate::errors::*;

pub async fn send_pow_config(session: Session) -> ServiceResult<impl Responder> {
    let config = PoWConfig::new(&session)?;
    debug!("PoW generated: {:#?}", &config);
    Ok(HttpResponse::Ok().json(config))
}
