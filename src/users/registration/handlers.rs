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

use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;

use super::models::User;
use super::payload::UnvalidatedRegisterCreds;
use crate::errors::*;
use crate::pow::PoWConfig;

pub async fn sign_up(
    session: Session,
    creds: web::Json<UnvalidatedRegisterCreds>,
    db_pool: web::Data<Pool>,
) -> ServiceResult<impl Responder> {
    PoWConfig::verify_pow(&session, &creds.pow)?;
    let processed_creds: User = creds.process()?.into();
    let new_user = processed_creds.add_user(db_pool).await?;
    debug!("{:?}", new_user);
    Ok(HttpResponse::Ok()
        .set_header(actix_web::http::header::CONNECTION, "close")
        .finish())
}
