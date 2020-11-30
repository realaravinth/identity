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
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_session::Session;
use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use redis_async::{resp::RespValue, resp_array};

use super::{Counter, PoWConfig, Visitor};
use crate::errors::*;
use crate::{Data, POW_SESSION_DURATION};

#[get("/api/pow")]
async fn get_pow(session: Session, data: web::Data<Data>) -> ServiceResult<impl Responder> {
    let session_id = session.get::<String>("PoW");
    if let Some(_id) = session_id? {
        Err(ServiceError::PoWRequired)
    } else {
        let phrase: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
        session.set("PoW", &phrase)?;
        let difficulty = data.counter_addr.send(Visitor).await.unwrap();
        data.redis_addr
            .send(Command(resp_array![
                "SET",
                &phrase,
                difficulty.to_string(),
                "EX",
                POW_SESSION_DURATION.to_string()
            ]))
            .await
            .unwrap()
            .unwrap();
        let config = PoWConfig::new(&phrase, difficulty);
        debug!("PoW generated: {:#?}", &config);
        Ok(HttpResponse::Ok().json(config))
    }
}

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(get_pow);
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test};

    use crate::test::DATA;

    #[actix_rt::test]
    async fn get_pow_works() {
        let mut app = test::init_service(crate::create_app().data(DATA.clone())).await;

        let mut response = test::call_service(
            &mut app,
            test::TestRequest::get().uri("/api/pow").to_request(),
        )
        .await;

        assert!(response.status().is_success(), "pow works");

        let cookie = response.response().cookies().next().unwrap().to_owned();

        response = test::call_service(
            &mut app,
            test::TestRequest::get()
                .cookie(cookie.clone())
                .uri("/api/pow")
                .to_request(),
        )
        .await;

        assert_eq!(response.status(), StatusCode::PAYMENT_REQUIRED, "pow works");
    }
}
