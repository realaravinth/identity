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
use actix_web::{post, web, HttpResponse, Responder};

use super::payload::LoginCreds;
use crate::errors::ServiceResult;
use crate::Data;

#[post("/api/signin")]
pub async fn sign_in(
    session: Session,
    creds: web::Json<LoginCreds>,
    data: web::Data<Data>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    creds.verify(&data.pool).await?;
    id.remember(creds.get_username());
    Ok(HttpResponse::Ok().finish())
}

#[post("/api/signout")]
pub async fn sign_out(id: Identity) -> ServiceResult<impl Responder> {
    if let Some(_) = id.identity() {
        id.forget();
    }
    Ok(HttpResponse::Ok())
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_in).service(sign_out);
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::{
        http::{header, Cookie, StatusCode},
        test,
    };

    use crate::pow::Visitor;
    use pow_sha256::PoW;
    use serde_json;

    use crate::pow::get_difficulty;
    use crate::pow::DIFFICULTY;

    static USERNAME: &str = "a";
    static PASSWORD: &str = "password";
    static HASH: &str = "$argon2i$v=19$m=656,t=5,p=12$b0p3MnZIbDRwRzUzTDRhZW9weWpBWFc5ZkxUREN5eGE$57mYgK/vkOFFlbh1QMttQ1eBUrbkYdPawmkmQwevziw";
    static EMAIL: &str = "batman@we.com";

    //    #[derive(Deserialize, Debug)]
    //    struct pow {
    //        pub PoW: String,
    //    }
    //
    use crate::test::DATA;

    //    async fn pow_helper() -> Cookie {
    //        let data = Data::default();
    //        let mut app = test::init_service(crate::create_app().data(data.clone())).await;
    //
    //        let response = test::call_service(
    //            &mut app,
    //            test::TestRequest::get().uri("/api/pow").to_request(),
    //        )
    //        .await;
    //
    //        // This statement borrows response(var name)
    //        // So can't extract the returned json with read_body_json()
    //        let cookie = response.response().cookies().next().unwrap().to_owned();
    //
    //        let difficulty = u128::max_value() - u128::max_value() / 100_000;
    //        let pow_secret = get_cookie(&cookie);
    //        let pow = PoW::prove_work(&pow_secret.as_bytes().to_vec(), difficulty).unwrap();
    //
    //        cookie
    //    }

    //    fn get_cookie(cookie: &Cookie) -> String {
    //        let value: String = cookie.value().into();
    //        let a: Vec<&str> = value.split('=').collect();
    //        let value: pow = serde_json::from_str(a[1]).unwrap();
    //        let pow_val: Vec<&str> = value.PoW.split('"').collect();
    //
    //        pow_val[1].to_string()
    //    }
    //
    #[actix_rt::test]
    async fn sign_in_works() {
        let data = Data::default();
        let mut app = test::init_service(crate::create_app().data(data.clone())).await;

        //        let response = test::call_service(
        //            &mut app,
        //            test::TestRequest::get().uri("/api/pow").to_request(),
        //        )
        //        .await;
        //
        //        // This statement borrows response(var name)
        //        // So can't extract the returned json with read_body_json()
        //        let cookie = response.response().cookies().next().unwrap().to_owned();
        //
        //        let pow_secret = get_cookie(&cookie);

        //        // So had to implement a hackish cookie parser like
        //        // that of Session::get(from actix_session)
        //        let value: String = cookie.value().into();
        //        let a: Vec<&str> = value.split('=').collect();
        //        let value: pow = serde_json::from_str(a[1]).unwrap();
        //        let pow_val: Vec<&str> = value.PoW.split('"').collect();
        //
        //        let pow_secret = &pow_val[1];

        //        let difficulty = u128::max_value() - u128::max_value() / 100_000;
        //
        //        let pow = PoW::prove_work(&pow_secret.as_bytes().to_vec(), difficulty).unwrap();

        let payload = serde_json::to_string(&LoginCreds {
            username: USERNAME.into(),
            password: PASSWORD.into(),
        })
        .unwrap();

        let req = test::TestRequest::post()
            .uri("/api/signin")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();

        let response = test::call_service(&mut app, req).await;

        println!("{}", response.status());

        assert!(response.status().is_success(), "signin works");
    }

    #[actix_rt::test]
    async fn sign_out() {
        let data = Data::default();
        let mut app = test::init_service(crate::create_app().data(data.clone())).await;

        let payload = serde_json::to_string(&LoginCreds {
            username: USERNAME.into(),
            password: PASSWORD.into(),
        })
        .unwrap();

        let req = test::TestRequest::post()
            .uri("/api/signin")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();

        let response = test::call_service(&mut app, req).await;

        let cookie = response.response().cookies().next().unwrap().to_owned();

        let req = test::TestRequest::post()
            .uri("/api/signout")
            .cookie(cookie)
            .to_request();

        let response = test::call_service(&mut app, req).await;

        println!("response: {}", response.status());
        assert!(response.status().is_success(), "sign out works");
    }

    #[actix_rt::test]
    async fn nonexistent_username() {
        let data = Data::default();
        let mut app = test::init_service(crate::create_app().data(data.clone())).await;

        let username_doesnt_exist = serde_json::to_string(&LoginCreds {
            username: "aaaa".into(),
            password: PASSWORD.into(),
        })
        .unwrap();

        let req = test::TestRequest::post()
            .uri("/api/signin")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(username_doesnt_exist)
            .to_request();

        let mut response = test::call_service(&mut app, req).await;

        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "username doesnt exist works"
        );
    }

    #[actix_rt::test]
    async fn signin_wrong_password() {
        let data = Data::default();
        let mut app = test::init_service(crate::create_app().data(data.clone())).await;

        let wrong_password = serde_json::to_string(&LoginCreds {
            username: USERNAME.into(),
            password: "aaa".into(),
        })
        .unwrap();

        let req = test::TestRequest::post()
            .uri("/api/signin")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(wrong_password)
            .to_request();

        let mut response = test::call_service(&mut app, req).await;

        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "wrong password works"
        );
    }
}
