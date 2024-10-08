use common::SESSION_ID_LENGTH;
use reqwest::{
    blocking::Client,
    header::{
        ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS,
        ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    },
    StatusCode,
};
use rocket::form::validate::Len;
use rust_template::{
    dto::NewUserResponseDto,
    errors::{ApiError, AuthError},
};
use serde_json::{from_value, json, Value};

use crate::common::{create_test_user, delete_test_user, generate_test_token};

pub mod common;

#[test]
fn when_credentials_correct_then_login_success() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "1234";
    let output = create_test_user(&username, &email, password, "viewer", &true.to_string());

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "email":email,
            "password":password,
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(output);

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn when_credentials_correct_then_login_returns_token() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "1234";
    let output = create_test_user(&username, &email, password, "viewer", &true.to_string());

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "email":email,
            "password":password,
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(output);

    let json: Value = response.json().unwrap();
    assert_eq!(json["token"].as_str().len(), 128);
}

#[test]
fn when_password_is_wrong_then_login_failed() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let output = create_test_user(&username, &email, "1234", "viewer", &true.to_string());

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "email":email,
            "password":"wrong_password"
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(output);

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn when_email_is_wrong_then_login_failed() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "1234";
    let output = create_test_user(&username, &email, password, "viewer", &true.to_string());

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "email":"wrong_email",
            "password":password
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(output);

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn when_user_exist_and_unconfirmed_then_login_return_unconfirmed_error() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "1234";
    let output = create_test_user(&username, &email, password, "viewer", &false.to_string());

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "email":email,
            "password":password,
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(output);

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::UnconfirmedUser.value())
}

#[test]
fn when_credentials_correct_and_available_then_signup_success() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "123456aA";

    let client = Client::new();

    let response = client
        .post(format!("{}/signup", common::APP_HOST))
        .json(&json!({
            "username":username,
            "email": email,
            "password":password
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[test]
fn when_credentials_ok_then_signup_returns_username_and_email() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "123456aA";

    let client = Client::new();

    let response = client
        .post(format!("{}/signup", common::APP_HOST))
        .json(&json!({
            "username":username,
            "email": email,
            "password":password
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let user: NewUserResponseDto = from_value(json).unwrap();

    assert_eq!(user, NewUserResponseDto { username, email });
}

#[test]
fn when_user_exist_then_signup_failed() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "123456aA";
    let output = create_test_user(&username, &email, password, "viewer", &true.to_string());

    let client = Client::new();

    let response = client
        .post(format!("{}/signup", common::APP_HOST))
        .json(&json!({
            "username":username,
            "email": email,
            "password":password
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(output);

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn when_user_exist_and_not_confirmed_then_returns_unconfirmed_user_error() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "123456aA";
    let output = create_test_user(&username, &email, password, "viewer", &false.to_string());

    let client = Client::new();

    let response = client
        .post(format!("{}/signup", common::APP_HOST))
        .json(&json!({
            "username":username,
            "email": email,
            "password":password
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(output);

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::UnconfirmedUser.value());
}

#[test]
fn when_username_exist_then_signup_returns_username_unavailable_error() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "123456aA";
    let output = create_test_user(&username, &email, password, "viewer", &true.to_string());

    let client = Client::new();

    let response = client
        .post(format!("{}/signup", common::APP_HOST))
        .json(&json!({
            "username":username,
            "email": "notExistedEmail@gmail.com",
            "password":password
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(output);

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::UnavailableUsername.value());
}

#[test]
fn when_inconsistent_username_then_signup_returns_invalid_username_error() {
    let username = format!("test_viewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "123456aA";

    let client = Client::new();

    let response = client
        .post(format!("{}/signup", common::APP_HOST))
        .json(&json!({
            "username":username,
            "email": email,
            "password":password
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidUsername.value());
}

#[test]
fn when_username_to_short_then_signup_returns_invalid_username_error() {
    let username = "te".to_string();
    let email = format!("{}@gmail.com", username);
    let password = "123456aA";

    let client = Client::new();

    let response = client
        .post(format!("{}/signup", common::APP_HOST))
        .json(&json!({
            "username":username,
            "email": email,
            "password":password
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidUsername.value());
}

#[test]
fn when_email_exist_then_signup_returns_email_in_use_error() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "123456aA";
    let output = create_test_user(&username, &email, password, "viewer", &true.to_string());

    let client = Client::new();

    let response = client
        .post(format!("{}/signup", common::APP_HOST))
        .json(&json!({
            "username":"availableUsername",
            "email": email,
            "password":password
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(output);

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::EmailInUse.value());
}

#[test]
fn when_email_without_at_sign_then_signup_returns_email_invalid_error() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}gmail.com", username);
    let password = "123456aA";

    let client = Client::new();

    let response = client
        .post(format!("{}/signup", common::APP_HOST))
        .json(&json!({
            "username":"availableUsername",
            "email": email,
            "password":password
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidEmail.value());
}

#[test]
fn when_email_without_domain_then_signup_returns_email_invalid_error() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}gmailcom", username);
    let password = "123456aA";

    let client = Client::new();

    let response = client
        .post(format!("{}/signup", common::APP_HOST))
        .json(&json!({
            "username":"availableUsername",
            "email": email,
            "password":password
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidEmail.value());
}

#[test]
fn when_password_to_short_then_signup_returns_invalid_password_error() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "12345";

    let client = Client::new();

    let response = client
        .post(format!("{}/signup", common::APP_HOST))
        .json(&json!({
            "username":username,
            "email": email,
            "password":password
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidPassword.value());
}

#[test]
fn when_inconsistent_password_then_signup_returns_invalid_password_error() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "abc123";

    let client = Client::new();

    let response = client
        .post(format!("{}/signup", common::APP_HOST))
        .json(&json!({
            "username":username,
            "email": email,
            "password":password
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidPassword.value());
}

#[test]
fn when_email_correct_and_exists_password_reset_success() {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let password = "123456aA";
    let output = create_test_user(&username, &email, password, "viewer", &true.to_string());

    let client = Client::new();

    let response = client
        .post(format!("{}/password_reset", common::APP_HOST))
        .json(&json!({
            "email": email,
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(output);

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn when_email_is_wrong_then_password_reset_returns_invalid_email_error() {
    let client = Client::new();

    let response = client
        .post(format!("{}/password_reset", common::APP_HOST))
        .json(&json!({
            "email": "wrong_email.gmail.com",
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidEmail.value());
}

#[test]
fn when_email_is_not_present_then_password_reset_returns_email_not_exist_error() {
    let email = format!("testViewer{}@gmail.com", rand::random::<u32>());
    let client = Client::new();

    let response = client
        .post(format!("{}/password_reset", common::APP_HOST))
        .json(&json!({
            "email": email,
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::EmailNotExist.value());
}

#[test]
fn when_password_wrong_then_password_failed() {
    let client = Client::new();
    let token = generate_test_token(SESSION_ID_LENGTH);

    let response = client
        .put(format!("{}/password/{token}", common::APP_HOST))
        .json(&json!({
            "password":"1234",
            "confirmation":"1234"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn when_password_wrong_then_password_returns_invalid_password_error() {
    let client = Client::new();
    let token = generate_test_token(SESSION_ID_LENGTH);

    let response = client
        .put(format!("{}/password/{token}", common::APP_HOST))
        .json(&json!({
            "password":"1234",
            "confirmation":"1234"
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidPassword.value())
}

#[test]
fn when_confirmation_is_not_match_then_password_returns_invalid_password_error() {
    let client = Client::new();
    let token = generate_test_token(SESSION_ID_LENGTH);

    let response = client
        .put(format!("{}/password/{token}", common::APP_HOST))
        .json(&json!({
            "password":"123456aA",
            "confirmation":"123456aB"
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidPassword.value())
}

#[test]
fn when_token_length_is_incorrect_then_password_returns_invalid_token_error() {
    let client = Client::new();
    let token = generate_test_token(64);

    let response = client
        .put(format!("{}/password/{token}", common::APP_HOST))
        .json(&json!({
            "password":"123456aA",
            "confirmation":"123456aA"
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidToken.value())
}

#[test]
fn when_token_wrong_or_expired_then_password_returns_unauthorized_status() {
    let client = Client::new();
    let token = generate_test_token(SESSION_ID_LENGTH);

    let response = client
        .put(format!("{}/password/{token}", common::APP_HOST))
        .json(&json!({
            "password":"123456aA",
            "confirmation":"123456aA"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED)
}

#[test]
fn when_token_missed_then_password_returns_not_found_status() {
    let client = Client::new();

    let response = client
        .put(format!("{}/password/", common::APP_HOST))
        .json(&json!({
            "password":"123456aA",
            "confirmation":"123456aA"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND)
}

#[test]
fn when_any_request_is_received_then_response_contains_cors_headers() {
    let client = Client::new();

    let response = client.get(common::APP_HOST.to_string()).send().unwrap();

    let headers = response.headers();

    assert_eq!(headers.get(ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(), "*");
    assert_eq!(headers.get(ACCESS_CONTROL_ALLOW_HEADERS).unwrap(), "*");
    assert_eq!(
        headers.get(ACCESS_CONTROL_ALLOW_METHODS).unwrap(),
        "GET, POST, PUT, DELETE, PATCH"
    );
    assert_eq!(
        headers.get(ACCESS_CONTROL_ALLOW_CREDENTIALS).unwrap(),
        "true"
    );
}

#[test]
fn when_confirmation_token_invalid_then_confirm_returns_invalid_token_error() {
    let client = Client::new();
    let token = generate_test_token(SESSION_ID_LENGTH);

    let response = client
        .get(format!("{}/confirm/{token}", common::APP_HOST))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidToken.value())
}
