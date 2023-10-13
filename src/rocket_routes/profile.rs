use rocket::serde::json::{serde_json::json, Json, Value};
use rocket::{futures::TryFutureExt, http::Status, response::status::Custom};

use crate::{
    auth::{self, is_password_valid},
    dto::NewPasswordDto,
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
pub fn me(user: User) -> Value {
    json!(user)
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
