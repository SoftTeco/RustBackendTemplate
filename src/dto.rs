use chrono::{NaiveDate, NaiveDateTime};
use utoipa::ToSchema;

/// New user request body
#[derive(serde::Serialize, ToSchema)]
pub struct NewUserDto {
    /// Unique username (at least 3 characters, ascii alphanumeric only)
    #[schema(example = "gunrock")]
    pub username: String,
    /// Unique email address
    #[schema(example = "gunrockg@gmail.com")]
    pub email: String,
    /// Password (at least 6 characters, at least one uppercase)
    #[schema(example = "123456aA")]
    pub password: String,
}

/// New user response body
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, ToSchema)]
pub struct NewUserResponseDto {
    #[schema(example = "gunrock")]
    pub username: String,
    #[schema(example = "gunrockg@gmail.com")]
    pub email: String,
}

/// Login credentials request body
#[derive(serde::Deserialize, ToSchema)]
pub struct CredentialsDto {
    #[schema(example = "gunrockg@gmail.com")]
    pub email: String,
    #[schema(example = "123456aA")]
    pub password: String,
}

/// Reset password request body
#[derive(serde::Deserialize, ToSchema)]
pub struct ResetPasswordEmailDto {
    /// Registered email address
    #[schema(example = "gunrockg@gmail.com")]
    pub email: String,
}

/// New password request body
#[derive(serde::Deserialize, ToSchema)]
pub struct NewPasswordDto {
    /// New password (at least 6 characters, at least one uppercase)
    #[schema(example = "123456aA")]
    pub password: String,
    /// Password confirmation
    #[schema(example = "123456aA")]
    pub confirmation: String,
}

/// Login response body
#[derive(serde::Serialize, ToSchema)]
pub struct AuthTokenDto {
    /// Token used for authentication
    #[schema(
        example = "pJWStSthAOTYSJIwPGjJBNVkFI0sKDmd8h2oZC1aFT0n1hbtbUJUJdahMEexCdw3pRw5qbG4KiQsNluT5c4H9FamBjxPp6ZsCYK3qduafOIzusbgOnUOd8LMyIJ1R39n"
    )]
    pub token: String,
}

/// User profile response body
#[derive(serde::Serialize, ToSchema)]
pub struct UserProfileDto {
    #[schema(example = 42)]
    pub id: i32,
    #[schema(example = "falcon")]
    pub username: String,
    #[schema(example = "falcon@gmail.com")]
    pub email: String,
    #[schema(example = "Edward")]
    pub first_name: Option<String>,
    #[schema(example = "Falcon")]
    pub last_name: Option<String>,
    #[schema(example = "Great Britain")]
    pub country: Option<String>,
    #[schema(value_type=Option<Vec<String>>,example="1970-01-01")]
    pub birth_date: Option<NaiveDate>,
    #[schema(value_type=Vec<String>,example="2023-10-12T10:00:14.930859")]
    pub created_at: NaiveDateTime,
    #[schema(value_type=Vec<String>,example="2024-08-21T13:35:16.389450")]
    pub updated_at: NaiveDateTime,
}

/// User profile update body
#[derive(serde::Deserialize, ToSchema)]
pub struct UpdateUserDto {
    #[schema(example = "Edward")]
    pub first_name: Option<String>,
    #[schema(example = "Falcon")]
    pub last_name: Option<String>,
    #[schema(example = "Great Britain")]
    pub country: Option<String>,
    #[schema(value_type=Option<Vec<String>>,example="1970-01-01")]
    pub birth_date: Option<NaiveDate>,
}
