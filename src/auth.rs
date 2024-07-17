use argon2::password_hash::{Error, SaltString};
use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};

use crate::dto::CredentialsDto;
use crate::models::NewUser;
use crate::{errors::AuthError, models::User};

pub const SESSION_LIFE_TIME: usize = 60 * 60 * 24;
pub const SESSION_ID_LENGTH: usize = 128;
pub const RESET_TOKEN_LIFE_TIME: usize = 60 * 60;
pub const SESSIONS_KEY_PREFIX: &str = "sessions";
pub const RESET_TOKEN_KEY_PREFIX: &str = "reset_token";
pub const RESET_PASSWORD_PATH: &str = "reset_password";
pub const CONFIRM_TOKEN_LIFE_TIME: usize = 60 * 60 * 24;
pub const CONFIRM_TOKEN_KEY_PREFIX: &str = "confirm_token";
pub const CONFIRM_EMAIL_PATH: &str = "confirm";
const MIN_PASSWORD_LENGTH: usize = 6;
const MIN_USERNAME_LENGTH: usize = 3;

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);
    let argon = argon2::Argon2::default();
    let password_hash = argon.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn authorize_user(user: &User, credentials: &CredentialsDto) -> Result<String, Error> {
    let db_hash = PasswordHash::new(&user.password)?;
    let argon = argon2::Argon2::default();
    argon.verify_password(credentials.password.as_bytes(), &db_hash)?;

    let session_id = generate_token(SESSION_ID_LENGTH);
    Ok(session_id)
}

pub fn generate_token(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn validate_signup_credentials(credentials: &NewUser) -> Result<(), AuthError> {
    if !is_username_valid(&credentials.username) {
        return Err(AuthError::InvalidUsername);
    }
    if !is_email_valid(&credentials.email) {
        return Err(AuthError::InvalidEmail);
    }
    if !is_password_valid(&credentials.password) {
        return Err(AuthError::InvalidPassword);
    }
    Ok(())
}

pub fn is_username_valid(username: &str) -> bool {
    username.len() >= MIN_USERNAME_LENGTH && username.chars().all(|c| c.is_ascii_alphanumeric())
}

pub fn is_password_valid(password: &str) -> bool {
    password.len() >= MIN_PASSWORD_LENGTH
        && password.chars().all(|c| c.is_ascii())
        && password.chars().any(|c| c.is_ascii_uppercase())
}

pub fn is_email_valid(email: &str) -> bool {
    if email.chars().any(|c| !c.is_ascii()) {
        return false;
    }
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let domain_parts: Vec<&str> = parts[1].split('.').collect();
    if domain_parts.len() < 2 {
        return false;
    }
    for part in parts.iter().chain(domain_parts.iter()) {
        if part.is_empty() {
            return false;
        }
    }
    true
}
