pub mod authorization;

use diesel::PgConnection;
use rocket::http::hyper::header;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use rocket::Request;

use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::{deadpool_redis, Connection, Database};

use crate::models::User;
use crate::repositories::UserRepository;

const AUTH_TYPE: &str = "Bearer";

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConnection(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConnection(deadpool_redis::Pool);

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request
            .headers()
            .get_one(header::AUTHORIZATION.as_str())
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == AUTH_TYPE);

        if let Some(header_value) = auth_header {
            let mut cache = request
                .guard::<Connection<CacheConnection>>()
                .await
                .expect("Cannot connect to redis in request guard");

            let db = request
                .guard::<DbConnection>()
                .await
                .expect("Cannot connect to postgres in request guard");

            let result = cache
                .get::<_, i32>(format!("sessions/{}", header_value[1]))
                .await;

            if let Ok(user_id) = result {
                return match db.run(move |c| UserRepository::find(c, user_id)).await {
                    Ok(user) => Outcome::Success(user),
                    _ => Outcome::Failure((Status::Unauthorized, ())),
                };
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
