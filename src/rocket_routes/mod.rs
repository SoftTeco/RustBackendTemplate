pub mod authorization;

use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use chrono::Utc;
use diesel::PgConnection;
use reqwest::ClientBuilder;
use rocket::http::hyper::header;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use rocket::Request;

use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::{deadpool_redis, Connection, Database};

use crate::auth::SESSIONS_KEY_PREFIX;
use crate::models::User;
use crate::repositories::UserRepository;

pub const DEEP_LINK_HOST: &str = "template.softteco.com.deep_link";
pub const DEEP_LINK_SCHEME: &str = "https";
const AUTH_TYPE: &str = "Bearer";
const IP_GEOLOCATION_API_URI: &str = "https://freeipapi.com/api/json";
const IP_GEOLOCATION_DURATION: u64 = 5;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConnection(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConnection(deadpool_redis::Pool);

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Internal Server Error"))
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
                .get::<_, i32>(format!("{}/{}", SESSIONS_KEY_PREFIX, header_value[1]))
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

pub async fn get_client_info(
    client_addr: SocketAddr,
) -> Result<String, Box<dyn std::error::Error>> {
    let date_time = Utc::now().format("%d %B %Y, %H:%M UTC").to_string();

    if (client_addr.ip()) == Ipv4Addr::LOCALHOST {
        return Ok(format!("{} at {}", client_addr.ip(), date_time));
    }

    let get_location_url = format!("{}/{}", IP_GEOLOCATION_API_URI, client_addr.ip());

    let timeout = Duration::new(IP_GEOLOCATION_DURATION, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;

    let response = client.get(&get_location_url).send().await.unwrap();
    let json: Value = response.json().await.unwrap();

    let city = json.get("cityName").unwrap();
    let country = json.get("countryCode").unwrap();

    Ok(format!(
        "{}, {}, {} at {}",
        client_addr.ip(),
        city,
        country,
        date_time
    ))
}
