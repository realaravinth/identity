/*
* Wagon is an independent mailing list manager
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




use actix_identity::Identity;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use futures;

use super::create_new_user;
use super::{Creds, NewCreds};
use crate::errors::ServiceResult;
use crate::pow::verify_pow;

pub async fn sign_up(
    session: Session,
    creds: web::Json<NewCreds>,
) -> ServiceResult<impl Responder> {
    let new_creds = creds.into_inner();
    let pow = &new_creds.pow;
    verify_pow(&session, &pow).await?;
    create_new_user(&new_creds.creds.username, &new_creds.creds.password).await?;
    Ok(HttpResponse::Ok()
        .set_header(actix_web::http::header::CONNECTION, "close")
        .finish())
}

pub async fn sign_in(
    session: Session,
    creds: web::Json<Creds>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    unimplemented!();
    Ok(HttpResponse::Ok().finish())
}

pub async fn sign_out(id: Identity) -> ServiceResult<impl Responder> {
    id.forget();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("You are successfully signed out"))
}
