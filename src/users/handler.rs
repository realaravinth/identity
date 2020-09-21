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
