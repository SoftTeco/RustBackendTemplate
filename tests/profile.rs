use common::get_logged_in_client;
use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::json;
use rust_template::errors::{ApiError, AuthError, ProfileError};
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
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    let (client, create_user_output) =
        get_logged_in_client(username.as_str(), email.as_str(), "viewer");

    let response = client
        .get(format!("{}/profile/me", common::APP_HOST))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();
    assert_eq!(
        (json.get("username").unwrap(), json.get("email").unwrap()),
        (&json!(username), &json!(email)),
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

#[test]
fn when_new_data_is_valid_then_update_user_returns_updated_user() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({
                "first_name": "Edward",
                "last_name": "Falcon",
                "country": "Great Britain",
                "birth_date": "1970-01-01"
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();

    assert_eq!(
        (
            json.get("first_name").unwrap(),
            json.get("last_name").unwrap(),
            json.get("country").unwrap(),
            json.get("birth_date").unwrap(),
        ),
        (
            &json!("Edward"),
            &json!("Falcon"),
            &json!("Great Britain"),
            &json!("1970-01-01"),
        ),
    );
}

#[test]
fn when_first_name_is_valid_then_update_user_returns_updated_user() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"first_name": "Edward"}))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();

    assert_eq!(json.get("first_name").unwrap(), &json!("Edward"));
}

#[test]
fn when_last_name_is_valid_then_update_user_returns_updated_user() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"last_name": "Falcon"}))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();

    assert_eq!(json.get("last_name").unwrap(), &json!("Falcon"));
}

#[test]
fn when_country_is_valid_then_update_user_returns_updated_user() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"country": "Great Britain"}))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();

    assert_eq!(json.get("country").unwrap(), &json!("Great Britain"));
}

#[test]
fn when_birth_date_is_valid_then_update_user_returns_updated_user() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"birth_date": "1970-01-01"}))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();

    assert_eq!(json.get("birth_date").unwrap(), &json!("1970-01-01"));
}

#[test]
fn when_name_is_invalid_or_empty_then_update_user_returns_invalid_first_name_error() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let _response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"first_name": "Edward"}))
        .send()
        .unwrap();

    let response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"first_name": ""}))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, ProfileError::InvalidFirstName.value());
}

#[test]
fn when_last_name_is_invalid_or_empty_then_update_user_returns_invalid_last_name_error() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let _response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"last_name": "Falcon 9"}))
        .send()
        .unwrap();

    let response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"last_name": ""}))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, ProfileError::InvalidLastName.value());
}

#[test]
fn when_country_is_invalid_or_empty_then_update_user_returns_invalid_country_error() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let _response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"country": "(qGreat Britain)"}))
        .send()
        .unwrap();

    let response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"country": ""}))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, ProfileError::InvalidCountry.value());
}

#[test]
fn when_birth_data_is_invalid_or_empty_then_update_user_returns_invalid_country_error() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let _response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"birth_date": "01-01-1970"}))
        .send()
        .unwrap();

    let response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"birth_date": ""}))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, ProfileError::InvalidBirthDate.value());
}

#[test]
fn if_values_are_null_patch_user_returns_user_with_null_values() {
    let (client, create_user_output) = get_client_with_logged_in_viewer();

    let _response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({
                "first_name": "Edward",
                "last_name": "Falcon",
                "country": "Great Britain",
                "birth_date": "1970-01-01",
        }))
        .send()
        .unwrap();

    let response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({
                "first_name": null,
                "last_name": null,
                "country": null,
                "birth_date":null,
        }))
        .send()
        .unwrap();

    // Cleanup
    delete_test_user(create_user_output);

    let json: Value = response.json().unwrap();

    assert_eq!(
        (
            json.get("first_name").unwrap(),
            json.get("last_name").unwrap(),
            json.get("country").unwrap(),
            json.get("birth_date").unwrap(),
        ),
        (&json!(null), &json!(null), &json!(null), &json!(null),),
    );
}

#[test]
fn when_session_is_not_active_then_update_user_returns_unauthorized_status() {
    let client = Client::new();

    let response = client
        .patch(format!("{}/profile/user", common::APP_HOST))
        .json(&json!({"first_name": "Edward"}))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    let error: ApiError = from_value(json).unwrap();

    assert_eq!(error, AuthError::InvalidToken.value());
}
