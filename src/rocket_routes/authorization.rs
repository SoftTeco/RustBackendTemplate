use super::{server_error, DbConnection};
use crate::{
    auth::{self, Credentials},
    models::User,
    repositories::{SessionRepository, UserRepository},
    rocket_routes::CacheConnection,
};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value},
};
use rocket_db_pools::Connection;

const CREDENTIALS_ERROR: &str = "Wrong credentials";

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    credentials: Json<Credentials>,
    db: DbConnection,
    cache: Connection<CacheConnection>,
) -> Result<Value, Custom<Value>> {
    let email = credentials.email.clone();
    let user = db
        .run(move |connection| {
            UserRepository::find_by_email(connection, &email).map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    Custom(Status::Unauthorized, json!(CREDENTIALS_ERROR))
                }
                _ => server_error(e.into()),
            })
        })
        .await?;

    let session_id = auth::authorize_user(&user, &credentials)
        .map_err(|_| Custom(Status::Unauthorized, json!(CREDENTIALS_ERROR)))?;

    SessionRepository::cache_session_id(&session_id, user.id, cache)
        .await
        .map(|_| json!({ "token": session_id }))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/me")]
pub fn me(user: User) -> Value {
    json!(user)
}
