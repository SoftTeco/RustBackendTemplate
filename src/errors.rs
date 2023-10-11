use utoipa::ToSchema;

#[derive(serde::Serialize, Debug, serde::Deserialize, PartialEq)]
pub struct ApiError {
    pub error_type: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, serde::Deserialize, PartialEq, ToSchema)]
pub enum AuthError {
    WrongCredentials,
    InvalidUsername,
    InvalidEmail,
    InvalidPassword,
    InvalidToken,
    UnavailableUsername,
    EmailInUse,
    EmailNotExist,
}

impl AuthError {
    pub fn value(&self) -> ApiError {
        const ERROR_TYPE: &str = "auth_error";
        match self {
            AuthError::WrongCredentials => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "wrong_credentials".to_string(),
                message: "Wrong credentials".to_string(),
            },
            AuthError::InvalidUsername => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "invalid_username".to_string(),
                message: "Invalid username".to_string(),
            },
            AuthError::InvalidEmail => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "invalid_email".to_string(),
                message: "Invalid email".to_string(),
            },
            AuthError::InvalidPassword => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "invalid_password".to_string(),
                message: "Invalid password".to_string(),
            },
            AuthError::InvalidToken => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "invalid_token".to_string(),
                message: "Invalid token".to_string(),
            },
            AuthError::UnavailableUsername => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "unavailable_username".to_string(),
                message: "Unavailable username".to_string(),
            },
            AuthError::EmailInUse => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "email_in_use".to_string(),
                message: "Email already in use".to_string(),
            },
            AuthError::EmailNotExist => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "email_not_exist".to_string(),
                message: "Email address is not associated with a personal user account".to_string(),
            },
        }
    }
}
