#[derive(serde::Serialize)]
pub struct NewUserDto {
    pub username: String,
    pub email: String,
}

#[derive(serde::Serialize, Debug)]
pub struct ApiError<'a> {
    pub error_type: &'a str,
    pub code: &'a str,
    pub message: &'a str,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    InvalidUsername,
    InvalidEmail,
    InvalidPassword,
    UnavailableUsername,
    EmailInUse,
}

impl AuthError {
    pub fn value(&self) -> ApiError {
        const ERROR_TYPE: &str = "auth_error";
        match self {
            AuthError::WrongCredentials => ApiError {
                error_type: ERROR_TYPE,
                code: "wrong_credentials",
                message: "Wrong credentials",
            },
            AuthError::InvalidUsername => ApiError {
                error_type: ERROR_TYPE,
                code: "invalid_username",
                message: "Invalid username",
            },
            AuthError::InvalidEmail => ApiError {
                error_type: ERROR_TYPE,
                code: "invalid_email",
                message: "Invalid email",
            },
            AuthError::InvalidPassword => ApiError {
                error_type: ERROR_TYPE,
                code: "invalid_password",
                message: "Invalid password",
            },
            AuthError::UnavailableUsername => ApiError {
                error_type: ERROR_TYPE,
                code: "unavailable_username",
                message: "Unavailable username",
            },
            AuthError::EmailInUse => ApiError {
                error_type: ERROR_TYPE,
                code: "email_in_use",
                message: "Email already in use",
            },
        }
    }
}
