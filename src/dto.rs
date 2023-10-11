#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct NewUserDto {
    pub username: String,
    pub email: String,
}

#[derive(serde::Deserialize)]
pub struct CredentialsDto {
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct ResetPasswordEmailDto {
    pub email: String,
}

#[derive(serde::Deserialize)]
pub struct NewPasswordDto {
    pub password: String,
    pub confirmation: String,
}
