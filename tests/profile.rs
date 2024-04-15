use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::json;
use rust_template::errors::{ApiError, AuthError};
use serde_json::{from_value, Value};

use crate::common::{
    delete_test_user, get_client_with_logged_in_editor, get_client_with_logged_in_viewer,
};

pub mod common;

#[test]
fn when_session_is_active_and_role_viewer_then_me_success() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let response = client
        .get(format!("{}/profile/me", common::APP_HOST))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn when_session_is_active_and_role_editor_then_me_success() {
    let (client, create_user_output) = get_client_with_logged_in_editor();

    let response = client
        .get(format!("{}/profile/me", common::APP_HOST))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn when_session_is_active_and_role_viewer_then_me_returns_username_and_email() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let response = client
        .get(format!("{}/profile/me", common::APP_HOST))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();
    assert_eq!(
        {
            json.get("username").unwrap();
            json.get("email").unwrap();
        },
        {
            "test_viewer";
            "test_viewer@gmail.com";
        },
    );
}

#[test]
fn when_session_is_active_and_me_success_then_password_is_none() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let response = client
        .get(format!("{}/profile/me", common::APP_HOST))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();
    assert!(json.get("password").is_none());
}

#[test]
fn when_session_is_not_active_me_failed() {
    let client = Client::new();

    let response = client
        .get(format!("{}/profile/me", common::APP_HOST))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidToken.value());
}

#[test]
fn when_session_is_active_and_payload_is_correct_then_password_success() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let response = client
        .put(format!("{}/profile/password", common::APP_HOST))
        .json(&json!({
            "password": "123456aA",
            "confirmation": "123456aA"
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn when_session_is_not_active_then_password_returns_unauthorized_status() {
    let client = Client::new();

    let response = client
        .put(format!("{}/profile/password", common::APP_HOST))
        .json(&json!({
            "password": "123456aA",
            "confirmation": "123456aA"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn when_session_is_active_and_password_is_wrong_then_password_failed() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let response = client
        .put(format!("{}/profile/password", common::APP_HOST))
        .json(&json!({
            "password": "1234",
            "confirmation": "123456aA"
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn when_session_is_active_and_password_is_wrong_then_password_returns_invalid_password_error() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let response = client
        .put(format!("{}/profile/password", common::APP_HOST))
        .json(&json!({
            "password": "1234",
            "confirmation": "123456aA"
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidPassword.value());
}

#[test]
fn when_session_is_active_and_confirmation_not_match_then_password_returns_invalid_password_err() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let response = client
        .put(format!("{}/profile/password", common::APP_HOST))
        .json(&json!({
            "password": "123456aA",
            "confirmation": "123456aB"
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidPassword.value());
}
