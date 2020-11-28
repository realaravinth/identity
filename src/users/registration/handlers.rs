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
use actix_web::{post, web, HttpResponse, Responder};
use deadpool_postgres::Pool;

use super::models::User;
use super::payload::UnvalidatedRegisterCreds;
use crate::errors::*;
use crate::pow::PoWConfig;

#[post("/api/signup")]
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

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_up);
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::{
        http::{header, StatusCode},
        test,
    };

    use pow_sha256::PoW;
    use serde_json;

    use crate::pow::DIFFICULTY;

    #[derive(Deserialize, Debug)]
    struct pow {
        pub PoW: String,
    }

    use crate::test::POOL;

    #[actix_rt::test]
    async fn sign_up_works() {
        let mut app = test::init_service(crate::create_app().data(POOL.pool.clone())).await;

        let response = test::call_service(
            &mut app,
            test::TestRequest::get().uri("/api/pow").to_request(),
        )
        .await;

        // This statement borrows response(var name)
        // So can't extract the returned json with read_body_json()
        let cookie = response.response().cookies().next().unwrap().to_owned();

        // So had to implement a hackish cookie parser like
        // that of Session::get(from actix_session)
        let value: String = cookie.value().into();
        let a: Vec<&str> = value.split('=').collect();
        let value: pow = serde_json::from_str(a[1]).unwrap();
        let pow_val: Vec<&str> = value.PoW.split('"').collect();

        let pow = PoW::prove_work(&pow_val[1].as_bytes().to_vec(), DIFFICULTY).unwrap();

        let random_username: String = "asdfasdf".into();
        let payload = serde_json::to_string(&UnvalidatedRegisterCreds {
            username: random_username.clone(),
            password: "asdfa".into(),
            email_id: Some("example@example.com".into()),
            pow,
        })
        .unwrap();

        let req = test::TestRequest::post()
            .cookie(cookie.clone())
            .uri("/api/signup")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload.clone())
            .to_request();

        let mut response = test::call_service(&mut app, req).await;

        assert!(response.status().is_success(), "pow works");

        response = test::call_service(
            &mut app,
            test::TestRequest::post()
                .cookie(cookie.clone())
                .uri("/api/signup")
                .header(header::CONTENT_TYPE, "application/json")
                .set_payload(payload.clone())
                .to_request(),
        )
        .await;

        assert_eq!(
            response.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "username exists works"
        );

        response = test::call_service(
            &mut app,
            test::TestRequest::post()
                .uri("/api/signup")
                .header(header::CONTENT_TYPE, "application/json")
                .set_payload(payload.clone())
                .to_request(),
        )
        .await;

        assert_eq!(
            response.status(),
            StatusCode::PAYMENT_REQUIRED,
            "PoW required"
        );

        let wrong_pow =
            PoW::prove_work(&pow_val[0].as_bytes().to_vec(), DIFFICULTY / 100_00).unwrap();

        let wrong_pow_payload = serde_json::to_string(&UnvalidatedRegisterCreds {
            username: random_username,
            password: "asdfa".into(),
            email_id: Some("example@example.com".into()),
            pow: wrong_pow,
        })
        .unwrap();

        response = test::call_service(
            &mut app,
            test::TestRequest::post()
                .uri("/api/signup")
                .cookie(cookie.clone())
                .header(header::CONTENT_TYPE, "application/json")
                .set_payload(wrong_pow_payload)
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "Invalid PoW");
    }
}
