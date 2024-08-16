use rocket::serde::json::Error;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{futures::TryFutureExt, http::Status, response::status::Custom};

use crate::dto::{NewPasswordDto, UpdateUserDto};
use crate::errors::ProfileError;
use crate::models::UpdatedUserInfo;
use crate::{
    auth::{self, is_password_valid},
    errors::AuthError,
    models::User,
    repositories::UserRepository,
    rocket_routes::DbConnection,
};

use super::server_error;

/// Get the current user's profile
#[utoipa::path(
    get,
    path = "/profile/me",
    responses(
        (status = 200, description = "OK", body = UserProfileDto),
        (status = 401, description = "Unauthorized", body = AuthError, examples(
            ("InvalidToken" = (summary = "errors::AuthError::InvalidToken", value = json!(AuthError::InvalidToken.value()))),
        )),
    ),
    security(("token"=[]))
)]
#[rocket::get("/profile/me")]
pub async fn me(user: Result<User, Value>) -> Result<Custom<Value>, Custom<Value>> {
    match user {
        Ok(user) => Ok(Custom(Status::Ok, json!(user))),
        Err(value) => Err(Custom(Status::Unauthorized, value)),
    }
}

/// Change the current user's password
#[utoipa::path(
    put,
    path = "/profile/password",
    request_body = NewPasswordDto,
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = AuthError, examples(
            ("InvalidPassword" = (summary = "errors::AuthError::InvalidPassword", value = json!(AuthError::InvalidPassword.value()))),
        )),
    ),
    security(("token"=[]))
)]
#[rocket::put("/profile/password", format = "json", data = "<password_dto>")]
pub async fn update_password(
    password_dto: Json<NewPasswordDto>,
    db: DbConnection,
    user: User,
) -> Result<Status, Custom<Value>> {
    let is_confirmation_equal = password_dto.password == password_dto.confirmation;
    if !is_confirmation_equal || !is_password_valid(&password_dto.password) {
        return Err(Custom(
            Status::BadRequest,
            json!(AuthError::InvalidPassword.value()),
        ));
    }

    let password_hash = auth::hash_password(password_dto.password.clone()).unwrap();

    db.run(move |connection| UserRepository::update_password(connection, user.id, &password_hash))
        .map_err(|e| server_error(e.into()))
        .map_ok(|_f| Status::Ok)
        .await
}

#[utoipa::path(
    patch,
    path = "/profile/user",
    request_body = UpdateUserDto,
    responses(
        (status = 200, description = "OK", body = UserProfileDto),
        (status = 401, description = "Unauthorized", body = AuthError, examples(
            ("InvalidToken" = (summary = "errors::AuthError::InvalidToken", value = json!(AuthError::InvalidToken.value()))),
        )),
        (status = 400, description = "Bad Request", body = ProfileError, examples(
            ("InvalidFirstName" = (summary = "errors::ProfileError::InvalidFirstName", value = json!(ProfileError::InvalidFirstName.value()))),
            ("InvalidLastName" = (summary = "errors::ProfileError::InvalidLastName", value = json!(ProfileError::InvalidLastName.value()))),
            ("InvalidCountry" = (summary = "errors::ProfileError::InvalidCountry", value = json!(ProfileError::InvalidCountry.value()))),
            ("InvalidBirthDate" = (summary = "errors::ProfileError::InvalidBirthDate", value = json!(ProfileError::InvalidBirthDate.value()))),
        )),
    ),
    security(("token"=[])),
)]
#[rocket::patch("/profile/user", format = "json", data = "<update_user_dto>")]
pub async fn update_user(
    update_user_dto: Result<Json<UpdateUserDto>, Error<'_>>,
    db: DbConnection,
    user: Result<User, Value>,
) -> Result<Custom<Value>, Custom<Value>> {
    if let Err(value) = user {
        return Err(Custom(Status::Unauthorized, value));
    };

    let err = |e: Box<ProfileError>| -> Result<Custom<Value>, Custom<Value>> {
        Err(Custom(Status::BadRequest, json!(e.value())))
    };

    let is_value_invalid = |field_value: String| {
        let trimmed_value = field_value.trim();

        trimmed_value.is_empty()
            || trimmed_value
                .chars()
                .any(|c| !(c.is_ascii_alphabetic() || c.is_ascii_whitespace()))
    };

    match update_user_dto {
        Ok(update_user_dto) => {
            if let Some(first_name) = update_user_dto.first_name.clone() {
                if is_value_invalid(first_name) {
                    return err(ProfileError::InvalidFirstName.into());
                }
            }

            if let Some(last_name) = update_user_dto.last_name.clone() {
                if is_value_invalid(last_name) {
                    return err(ProfileError::InvalidLastName.into());
                }
            }

            if let Some(country) = update_user_dto.country.clone() {
                if is_value_invalid(country) {
                    return err(ProfileError::InvalidCountry.into());
                }
            }

            let info = UpdatedUserInfo {
                first_name: update_user_dto.0.first_name.map(|v| v.trim().to_string()),
                last_name: update_user_dto.0.last_name.map(|v| v.trim().to_string()),
                country: update_user_dto.0.country.map(|v| v.trim().to_string()),
                birth_date: update_user_dto.0.birth_date,
            };

            db.run(move |connection| {
                UserRepository::update_user(connection, user.unwrap().id, info)
            })
            .map_err(|e| server_error(e.into()))
            .map_ok(|updated_user| Custom(Status::Ok, json!(updated_user)))
            .await
        }
        Err(_) => err(ProfileError::InvalidBirthDate.into()),
    }
}

/// Delete the current user's profile
#[utoipa::path(
    delete,
    path = "/profile/user",
    responses(
        (status = 204),
        (status = 401, description = "Unauthorized", body = AuthError, examples(
            ("InvalidToken" = (summary = "errors::AuthError::InvalidToken", value = json!(AuthError::InvalidToken.value()))),
        )),
    ),
    security(("token"=[]))
)]
#[rocket::delete("/profile/user")]
pub async fn delete_user(
    db: DbConnection,
    user: Result<User, Value>,
) -> Result<Status, Custom<Value>> {
    if let Err(value) = user {
        return Err(Custom(Status::Unauthorized, value));
    };

    db.run(move |connection| UserRepository::delete(connection, user.unwrap().id))
        .map_err(|e| server_error(e.into()))
        .map_ok(|_| Status::NoContent)
        .await
}
