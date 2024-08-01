pub mod authorization;
pub mod profile;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use chrono::Utc;
use diesel::PgConnection;
use reqwest::ClientBuilder;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::hyper::header;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use rocket::Request;

use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::{deadpool_redis, Connection, Database};

use crate::auth::SESSIONS_KEY_PREFIX;
use crate::errors::AuthError;
use crate::models::User;
use crate::repositories::UserRepository;

pub const DEEP_LINK_HOST: &str = "template.softteco.com.deep_link";
pub const DEEP_LINK_SCHEME: &str = "https";
const AUTH_TYPE: &str = "Bearer";
const IP_GEOLOCATION_API_URI: &str = "https://freeipapi.com/api/json";
const IP_GEOLOCATION_DURATION: u64 = 5;
const VIEWER_ADDRESS_HEADER: &str = "CloudFront-Viewer-Address";

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConnection(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConnection(deadpool_redis::Pool);

pub struct ClientAddr(IpAddr);

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("Internal Server Error: {}", e);
    Custom(Status::InternalServerError, json!("Internal Server Error"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientAddr {
    type Error = Value;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header = request.headers().get_one(VIEWER_ADDRESS_HEADER);
        match header {
            Some(address) => match address.parse::<SocketAddr>() {
                Ok(socket_addr) => Outcome::Success(ClientAddr(socket_addr.ip())),
                Err(e) => Outcome::Error((
                    Status::InternalServerError,
                    json!(format!("Unable to extract client IP address: {}", e)),
                )),
            },
            None => match request.client_ip() {
                Some(remote_addr) => Outcome::Success(ClientAddr(remote_addr)),
                None => Outcome::Error((
                    Status::InternalServerError,
                    json!("Unable to extract client IP address"),
                )),
            },
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Value;
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
                    _ => Outcome::Error((
                        Status::Unauthorized,
                        json!(AuthError::InvalidToken.value()),
                    )),
                };
            }
        }

        Outcome::Error((Status::Unauthorized, json!(AuthError::InvalidToken.value())))
    }
}

pub async fn get_client_info(client_addr: IpAddr) -> Result<String, Box<dyn std::error::Error>> {
    let date_time = Utc::now().format("%d %B %Y, %H:%M UTC").to_string();

    if (client_addr) == Ipv4Addr::LOCALHOST {
        return Ok(format!("{} at {}", client_addr, date_time));
    }

    let get_location_url = format!("{}/{}", IP_GEOLOCATION_API_URI, client_addr);

    let timeout = Duration::new(IP_GEOLOCATION_DURATION, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;

    let response = client.get(&get_location_url).send().await.unwrap();
    let json: Value = response.json().await.unwrap();

    let city = json.get("cityName").unwrap();
    let country = json.get("countryCode").unwrap();

    Ok(format!(
        "{}, {}, {} at {}",
        client_addr, city, country, date_time
    ))
}

/// Append CORS headers in responses
#[allow(clippy::let_unit_value)]
#[rocket::options("/<_route_args..>")]
pub fn options(_route_args: Option<std::path::PathBuf>) -> Status {
    Status::NoContent
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Append CORS headers in responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut rocket::Response<'r>) {
        res.set_raw_header("Access-Control-Allow-Origin", "*");
        res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
        res.set_raw_header("Access-Control-Allow-Headers", "*");
        res.set_raw_header("Access-Control-Allow-Credentials", "true");
    }
}
