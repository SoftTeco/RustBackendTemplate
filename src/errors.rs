pub enum AuthError {
    WrongCredentials,
    InvalidUsername,
    InvalidEmail,
    InvalidPassword,
    UnavailableUsername,
    EmailInUse,
}

impl AuthError {
    pub fn value(&self) -> &str {
        match self {
            AuthError::WrongCredentials => "Wrong credentials",
            AuthError::InvalidUsername => "Invalid username",
            AuthError::InvalidEmail => "Invalid email",
            AuthError::InvalidPassword => "Invalid password",
            AuthError::UnavailableUsername => "User name unavailable",
            AuthError::EmailInUse => "Email already in use",
        }
    }
}
