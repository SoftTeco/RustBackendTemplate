use reqwest::{blocking::Client, StatusCode};
use rocket::form::validate::Len;
use serde_json::{json, Value};

use crate::common::{
    create_test_user, get_client_with_logged_in_editor, get_client_with_logged_in_viewer,
};

pub mod common;

#[test]
fn when_session_is_active_and_role_viewer_then_me_success() {
    let client = get_client_with_logged_in_viewer();

    let response = client
        .get(format!("{}/me", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn when_session_is_active_and_role_editor_then_me_success() {
    let client = get_client_with_logged_in_editor();

    let response = client
        .get(format!("{}/me", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn when_session_is_active_and_role_viewer_then_me_returns_username_and_email() {
    let client = get_client_with_logged_in_viewer();

    let response = client
        .get(format!("{}/me", common::APP_HOST))
        .send()
        .unwrap();

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
    let client = get_client_with_logged_in_viewer();

    let response = client
        .get(format!("{}/me", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("password").is_none());
}

#[test]
fn when_session_is_not_active_me_failed() {
    let client = Client::new();

    let response = client
        .get(format!("{}/me", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn when_credentials_correct_then_login_success() {
    let username = "test_viewer";
    let email = format!("{}@gmail.com", username);
    let password = "1234";
    let output = create_test_user(&username, &email, &password, "viewer");

    print!("{:?}", output);

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "email":email,
            "password":password,
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn when_credentials_correct_then_login_returns_token() {
    let username = "test_viewer";
    let email = format!("{}@gmail.com", username);
    let password = "1234";
    let output = create_test_user(&username, &email, &password, "viewer");

    print!("{:?}", output);

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "email":email,
            "password":password,
        }))
        .send()
        .unwrap();

    let json: Value = response.json().unwrap();
    assert_eq!(json["token"].as_str().len(), 128);
}

#[test]
fn when_password_is_wrong_then_login_failed() {
    let username = "test_viewer";
    let email = format!("{}@gmail.com", username);
    let output = create_test_user(&username, &email, "1234", "viewer");

    print!("{:?}", output);

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "email":email,
            "password":"wrong_password"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn when_email_is_wrong_then_login_failed() {
    let username = "test_viewer";
    let email = format!("{}@gmail.com", username);
    let password = "1234";
    let output = create_test_user(&username, &email, password, "viewer");

    print!("{:?}", output);

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "email":"wrong_email",
            "password":password
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
