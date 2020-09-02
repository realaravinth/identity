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

use actix_web::web::{self, HttpResponse};

use super::{sign_in, sign_out, sign_up};
use crate::pow::send_pow_config;

#[cfg(not(tarpaulin_include))]
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/signin")
            .route(web::post().to(sign_in))
            .route(web::get().to(send_pow_config))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
    cfg.service(
        web::resource("/api/signup")
            .route(web::post().to(sign_up))
            .route(web::get().to(send_pow_config))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
    cfg.service(
        web::resource("/api/signout")
            .route(web::post().to(sign_out))
            .route(web::get().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
