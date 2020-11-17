/*
* Copyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as
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

use actix_identity::Identity;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};

use super::models::User;
use super::payload::Unvalidated_RegisterCreds;
use crate::errors::*;
use crate::pow::verify_pow;

pub async fn sign_up(
    session: Session,
    creds: web::Json<Unvalidated_RegisterCreds>,
) -> ServiceResult<impl Responder> {
    verify_pow(&session, &creds.pow).await?;
    let processed_creds: User = creds.process()?.into();
    debug!("{:?}", processed_creds);
    Ok(HttpResponse::Ok()
        .set_header(actix_web::http::header::CONNECTION, "close")
        .finish())
}
