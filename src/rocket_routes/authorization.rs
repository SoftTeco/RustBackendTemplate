use std::net::IpAddr;

use super::{server_error, ClientAddr, DbConnection, DEEP_LINK_HOST, DEEP_LINK_SCHEME};
use crate::{
    auth::{
        self, generate_token, is_email_valid, is_password_valid, validate_signup_credentials,
        CONFIRM_EMAIL_PATH, CONFIRM_TOKEN_KEY_PREFIX, CONFIRM_TOKEN_LIFE_TIME, RESET_PASSWORD_PATH,
        RESET_TOKEN_KEY_PREFIX, RESET_TOKEN_LIFE_TIME, SESSION_ID_LENGTH,
    },
    dto::{
        AuthTokenDto, CredentialsDto, NewPasswordDto, NewUserResponseDto, ResetPasswordEmailDto,
    },
    errors::AuthError,
    mail::{send_confirmation_email, send_reset_password_email},
    models::{NewUser, RoleCode},
    repositories::{SessionRepository, UserRepository},
    rocket_routes::CacheConnection,
};

use chrono::{TimeDelta, Utc};
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
use rocket_dyn_templates::{context, Template};

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
            ("UnconfirmedUser" = (summary = "errors::AuthError::UnconfirmedUser", value = json!(AuthError::UnconfirmedUser.value()))),

        )),
    )

)]
#[rocket::post("/signup", format = "json", data = "<credentials>")]
pub async fn signup(
    credentials: Json<NewUser>,
    db: DbConnection,
    cache: Connection<CacheConnection>,
    client_addr: ClientAddr,
) -> Result<Custom<Value>, Custom<Value>> {
    if let Err(e) = validate_signup_credentials(&credentials) {
        return Err(Custom(Status::BadRequest, json!(e.value())));
    }

    let email = credentials.email.clone();
    check_existence(email, &db).await?;

    let password_hash = auth::hash_password(credentials.password.clone()).unwrap();
    let new_user = NewUser {
        username: credentials.username.clone(),
        email: credentials.email.clone(),
        password: password_hash.to_string(),
    };

    let user = db
        .run(move |connection| {
            UserRepository::create(connection, new_user, vec![RoleCode::Viewer]).map_err(
                |e| match e {
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
                },
            )
        })
        .await?;

    let confirm_token = generate_token(SESSION_ID_LENGTH);

    SessionRepository::cache_token(
        &confirm_token,
        user.id,
        CONFIRM_TOKEN_KEY_PREFIX,
        CONFIRM_TOKEN_LIFE_TIME,
        cache,
    )
    .await
    .map_err(|e| server_error(e.into()))?;

    let base_url = std::env::var("BASE_URL").expect("Unable to read base URL from env");
    let link = format!("{base_url}/{CONFIRM_EMAIL_PATH}/{confirm_token}");

    send_confirmation_email(&user, link, client_addr.0).await;

    Ok(Custom(
        Status::Created,
        json!(NewUserResponseDto {
            username: user.username,
            email: user.email,
        }),
    ))
}

async fn check_existence(email: String, db: &DbConnection) -> Result<(), Custom<Value>> {
    let existing_user = db
        .run(move |connection| UserRepository::find_by_email(connection, &email).map_err(|_| ()))
        .await;

    if let Ok(user) = existing_user {
        if user.confirmed {
            return Ok(());
        }

        let current_time = Utc::now().naive_utc();
        let expiration_time = user
            .created_at
            .checked_add_signed(TimeDelta::try_seconds(CONFIRM_TOKEN_LIFE_TIME as i64).unwrap())
            .unwrap();

        if current_time > expiration_time {
            let _ = db
                .run(move |connection| {
                    UserRepository::delete(connection, user.id).map_err(|e| server_error(e.into()))
                })
                .await;
            return Ok(());
        } else {
            return Err(Custom(
                Status::BadRequest,
                json!(AuthError::UnconfirmedUser.value()),
            ));
        }
    }
    Ok(())
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
            ("UnconfirmedUser" = (summary = "errors::AuthError::UnconfirmedUser", value = json!(AuthError::UnconfirmedUser.value()))),
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

    if !user.confirmed {
        return Err(Custom(
            Status::BadRequest,
            json!(AuthError::UnconfirmedUser.value()),
        ));
    }

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
    client_addr: IpAddr,
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

    SessionRepository::cache_token(
        &reset_token,
        user.id,
        RESET_TOKEN_KEY_PREFIX,
        RESET_TOKEN_LIFE_TIME,
        cache,
    )
    .await
    .map_err(|e| server_error(e.into()))?;

    let deep_link =
        format!("{DEEP_LINK_SCHEME}://{DEEP_LINK_HOST}/{RESET_PASSWORD_PATH}/{reset_token}");

    send_reset_password_email(user, deep_link, client_addr).await;

    Ok(Status::Ok)
}

/// Change the password in the password reset flow
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
    token: &str,
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

    let user_id =
        UserRepository::find_id_by_temporary_token(token, RESET_TOKEN_KEY_PREFIX, &mut cache)
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

    SessionRepository::redeem_token(token, RESET_TOKEN_KEY_PREFIX, &mut cache)
        .map_err(|e| server_error(e.into()))
        .await?;

    Ok(Status::Ok)
}

/// Change user status to confirmed;
///
///Render the confirmation page and deep link (mobile version)
#[utoipa::path(
    get,
    path = "/confirm/{token}",
    params(("token" = String, Path, description = "The signup confirmation token",)),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = AuthError, examples(
            ("InvalidToken" = (summary = "errors::AuthError::InvalidToken", value = json!(AuthError::InvalidToken.value()))),
        ))
    )
)]
#[rocket::get("/confirm/<token>")]
pub async fn confirm_signup(
    token: &str,
    db: DbConnection,
    mut cache: Connection<CacheConnection>,
) -> Result<Template, Custom<Value>> {
    if token.len() != SESSION_ID_LENGTH {
        return Err(Custom(
            Status::BadRequest,
            json!(AuthError::InvalidToken.value()),
        ));
    }

    let user_id =
        UserRepository::find_id_by_temporary_token(token, CONFIRM_TOKEN_KEY_PREFIX, &mut cache)
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
                json!((AuthError::InvalidToken.value())),
            ),
            _ => server_error(e.into()),
        })
        .await?;

    if !user.confirmed {
        let _ = db
            .run(move |connection| UserRepository::confirm_signup(connection, user.id))
            .map_err(|e| server_error(e.into()))
            .await;
    }

    let deep_link = format!("{DEEP_LINK_SCHEME}://{DEEP_LINK_HOST}");
    let base_url = std::env::var("BASE_URL").expect("Unable to read base URL from env");
    let link = format!("{base_url}/{CONFIRM_EMAIL_PATH}");
    let context = context! {
     deep_link: &deep_link,
     redirect_link: &link
    };

    let template = Template::render("page/confirmation", context);

    Ok(template)
}
