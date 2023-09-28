use super::{server_error, DbConnection};
use crate::{
    auth::{self, validate_signup_credentials, Credentials},
    errors::AuthError,
    models::{NewUser, NewUserDto, RoleCode, User},
    repositories::{SessionRepository, UserRepository},
    rocket_routes::CacheConnection,
};

use diesel::result::DatabaseErrorKind;
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value},
};
use rocket_db_pools::Connection;

#[rocket::post("/signup", format = "json", data = "<credentials>")]
pub async fn signup(
    credentials: Json<NewUser>,
    db: DbConnection,
) -> Result<Custom<Value>, Custom<Value>> {
    if let Err(e) = validate_signup_credentials(&credentials) {
        return Err(Custom(Status::BadRequest, json!(e.value())));
    }

    let password_hash = auth::hash_password(credentials.password.clone()).unwrap();
    let new_user = NewUser {
        username: credentials.username.clone(),
        email: credentials.email.clone(),
        password: password_hash.to_string(),
    };

    db.run(move |connection| {
        UserRepository::create(connection, new_user, vec![RoleCode::Viewer])
            .map(|user| {
                Custom(
                    Status::Created,
                    json!(NewUserDto {
                        username: user.username,
                        email: user.email
                    }),
                )
            })
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    DatabaseErrorKind::UniqueViolation,
                    error_info,
                ) => match error_info.constraint_name() {
                    Some("users_email_key") => {
                        Custom(Status::BadRequest, json!(AuthError::EmailInUse.value()))
                    }
                    Some("users_username_key") => Custom(
                        Status::BadRequest,
                        json!(AuthError::UnavailableUsername.value()),
                    ),
                    _ => Custom(
                        Status::BadRequest,
                        json!(AuthError::WrongCredentials.value()),
                    ),
                },
                _ => server_error(e.into()),
            })
    })
    .await
}

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
                diesel::result::Error::NotFound => Custom(
                    Status::Unauthorized,
                    json!(AuthError::WrongCredentials.value()),
                ),
                _ => server_error(e.into()),
            })
        })
        .await?;

    let session_id = auth::authorize_user(&user, &credentials).map_err(|_| {
        Custom(
            Status::Unauthorized,
            json!(AuthError::WrongCredentials.value()),
        )
    })?;

    SessionRepository::cache_session_id(&session_id, user.id, cache)
        .await
        .map(|_| json!({ "token": session_id }))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/me")]
pub fn me(user: User) -> Value {
    json!(user)
}
