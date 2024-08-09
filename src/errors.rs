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
    UnconfirmedUser,
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
                message: "Token is invalid or expired".to_string(),
            },
            AuthError::UnavailableUsername => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "unavailable_username".to_string(),
                message: "Unavailable username".to_string(),
            },
            AuthError::UnconfirmedUser => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "unconfirmed_user".to_string(),
                message: "User has not confirmed the registration via e-mail link".to_string(),
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

#[derive(Debug, serde::Deserialize, PartialEq, ToSchema)]
pub enum ProfileError {
    InvalidFirstName,
    InvalidLastName,
    InvalidCountry,
    InvalidBirthDate,
}

impl ProfileError {
    pub fn value(&self) -> ApiError {
        const ERROR_TYPE: &str = "profile_error";
        match self {
            ProfileError::InvalidFirstName => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "invalid_first_name".to_string(),
                message: "Invalid first name".to_string(),
            },
            ProfileError::InvalidLastName => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "invalid_last_name".to_string(),
                message: "Invalid last name".to_string(),
            },
            ProfileError::InvalidCountry => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "invalid_country".to_string(),
                message: "Invalid country".to_string(),
            },
            ProfileError::InvalidBirthDate => ApiError {
                error_type: ERROR_TYPE.to_string(),
                code: "invalid_birth_date".to_string(),
                message: "Birth date must be in YYYY-MM-DD format".to_string(),
            },
        }
    }
}
