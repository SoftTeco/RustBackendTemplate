use std::net::SocketAddr;

use super::{server_error, DbConnection, DEEP_LINK_HOST, DEEP_LINK_SCHEME};
use crate::{
    auth::{self, generate_token, is_email_valid, validate_signup_credentials},
    auth::{is_password_valid, RESET_PASSWORD_PATH, SESSION_ID_LENGTH},
    dto::{
        AuthTokenDto, CredentialsDto, NewPasswordDto, NewUserResponseDto, ResetPasswordEmailDto,
    },
    errors::AuthError,
    mail::send_reset_password_email,
    models::{NewUser, RoleCode},
    repositories::{SessionRepository, UserRepository},
    rocket_routes::CacheConnection,
};

use diesel::result::DatabaseErrorKind;

use rocket::{
    futures::TryFutureExt,
    http::Status,
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value},
};
use rocket_db_pools::{
    deadpool_redis::redis::{ErrorKind, RedisError},
    Connection,
};

/// Signup with email, username and password
///
/// **Email** and *username* must be unique;
///
/// **Username** must be at least 3 characters long, and must contain only ascii alphanumeric characters;
///
/// **Password** must be at least 6 characters long, and must contain at least one uppercase letter, one lowercase letter.
#[utoipa::path(
    post,
    path = "/signup",
    request_body = NewUserDto,
    responses(
        (status = 200, description = "OK", body = NewUserResponseDto),
        (status = 400, description = "Bad Request", body = AuthError, examples(
            ("InvalidUsername" = (summary = "errors::AuthError::InvalidUsername", value = json!(AuthError::InvalidUsername.value()))),
            ("InvalidEmail" = (summary = "errors::AuthError::InvalidEmail", value = json!(AuthError::InvalidEmail.value()))),
            ("InvalidPassword" = (summary = "errors::AuthError::InvalidPassword", value = json!(AuthError::InvalidPassword.value()))),
            ("EmailInUse" = (summary = "errors::AuthError::EmailInUse", value = json!(AuthError::EmailInUse.value()))),
            ("UnavailableUsername" = (summary = "errors::AuthError::UnavailableUsername", value = json!(AuthError::UnavailableUsername.value()))),
            ("WrongCredentials" = (summary = "errors::AuthError::WrongCredentials", value = json!(AuthError::WrongCredentials.value()))),
        )),
    )

)]
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
                    json!(NewUserResponseDto {
                        username: user.username,
                        email: user.email,
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

/// Log in with the given credentials
///
/// Returns an auth token if successful.
#[utoipa::path(
    post,
    path = "/login",
    request_body = CredentialsDto,
    responses(
        (status = 200, description = "OK", body = AuthTokenDto),
        (status = 401, description = "Unauthorized", body = AuthError, examples(
            ("WrongCredentials" = (summary = "errors::AuthError::WrongCredentials", value = json!(AuthError::WrongCredentials.value()))),
            ("EmailNotExist" = (summary = "errors::AuthError::EmailNotExist", value = json!(AuthError::EmailNotExist.value()))),
        )),
    )
)]
#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    credentials: Json<CredentialsDto>,
    db: DbConnection,
    cache: Connection<CacheConnection>,
) -> Result<Value, Custom<Value>> {
    let email = credentials.email.clone();
    let user = db
        .run(move |connection| {
            UserRepository::find_by_email(connection, &email).map_err(|e| match e {
                diesel::result::Error::NotFound => Custom(
                    Status::Unauthorized,
                    json!(AuthError::EmailNotExist.value()),
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
        .map(|_| json!(AuthTokenDto { token: session_id }))
        .map_err(|e| server_error(e.into()))
}

/// Initiate sending a password reset email
///
/// If successful, a deep link with a reset token will be sent to the provided email address;
///
/// The token expires after 1 hour and can only be used once;
///
/// The deep link format: `https://template.softteco.com.deep_link/reset_password/{token}`.
#[utoipa::path(
    post,
    path = "/password_reset",
    request_body = ResetPasswordEmailDto,
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = AuthError, examples(
            ("InvalidEmail" = (summary = "errors::AuthError::InvalidEmail", value = json!(AuthError::InvalidEmail.value()))),
            ("EmailNotExist" = (summary = "errors::AuthError::EmailNotExist", value = json!(AuthError::EmailNotExist.value()))),
        ))
    )
)]
#[rocket::post("/password_reset", format = "json", data = "<email_dto>")]
pub async fn reset_password(
    email_dto: Json<ResetPasswordEmailDto>,
    db: DbConnection,
    cache: Connection<CacheConnection>,
    client_addr: SocketAddr,
) -> Result<Status, Custom<Value>> {
    if !is_email_valid(&email_dto.email) {
        return Err(Custom(
            Status::BadRequest,
            json!(AuthError::InvalidEmail.value()),
        ));
    }

    let user = db
        .run(move |connection| {
            UserRepository::find_by_email(connection, &email_dto.email).map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    Custom(Status::NotFound, json!(AuthError::EmailNotExist.value()))
                }
                _ => server_error(e.into()),
            })
        })
        .await?;

    let reset_token = generate_token(SESSION_ID_LENGTH);

    SessionRepository::cache_reset_token(&reset_token, user.id, cache)
        .await
        .map_err(|e| server_error(e.into()))?;

    let deep_link =
        format!("{DEEP_LINK_SCHEME}://{DEEP_LINK_HOST}/{RESET_PASSWORD_PATH}/{reset_token}");

    send_reset_password_email(user, deep_link, client_addr).await;

    Ok(Status::Ok)
}

/// Changing the password in the password reset flow
#[utoipa::path(
    put,
    path = "/password/{token}",
    params(("token" = String, Path, description = "The password reset token",)),
    request_body = NewPasswordDto,
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = AuthError, examples(
            ("InvalidToken" = (summary = "errors::AuthError::InvalidToken", value = json!(AuthError::InvalidToken.value()))),
            ("InvalidPassword" = (summary = "errors::AuthError::InvalidPassword", value = json!(AuthError::InvalidPassword.value()))),
            ("EmailNotExist" = (summary = "errors::AuthError::EmailNotExist", value = json!(AuthError::EmailNotExist.value()))),
        ))
    )
)]
#[rocket::put("/password/<token>", format = "json", data = "<password_dto>")]
pub async fn change_password(
    password_dto: Json<NewPasswordDto>,
    token: String,
    db: DbConnection,
    mut cache: Connection<CacheConnection>,
) -> Result<Status, Custom<Value>> {
    if token.len() != SESSION_ID_LENGTH {
        return Err(Custom(
            Status::BadRequest,
            json!(AuthError::InvalidToken.value()),
        ));
    }

    let is_confirmation_match = password_dto.password == password_dto.confirmation;
    if !is_confirmation_match || !is_password_valid(&password_dto.password) {
        return Err(Custom(
            Status::BadRequest,
            json!(AuthError::InvalidPassword.value()),
        ));
    }

    let user_id = UserRepository::find_id_by_temporary_token(&token, &mut cache)
        .map_err(|e: RedisError| match e.kind() {
            ErrorKind::TypeError => {
                Custom(Status::Unauthorized, json!(AuthError::InvalidToken.value()))
            }
            _ => server_error(e.into()),
        })
        .await?;

    let user = db
        .run(move |connection| UserRepository::find(connection, user_id))
        .map_err(|e| match e {
            diesel::result::Error::NotFound => Custom(
                Status::Unauthorized,
                json!((AuthError::EmailNotExist.value())),
            ),
            _ => server_error(e.into()),
        })
        .await?;

    let password_hash = auth::hash_password(password_dto.password.clone()).unwrap();

    let _ = db
        .run(move |connection| UserRepository::update_password(connection, user.id, &password_hash))
        .map_err(|e| server_error(e.into()))
        .await;

    SessionRepository::redeem_reset_token(&token, &mut cache)
        .map_err(|e| server_error(e.into()))
        .await?;

    Ok(Status::Ok)
}
