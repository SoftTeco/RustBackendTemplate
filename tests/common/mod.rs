use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::process::{Command, Output};

pub const APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn create_test_user(username: &str, email: &str, password: &str, role: &str) -> Output {
    Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg(username)
        .arg(email)
        .arg(password)
        .arg(role)
        .output()
        .unwrap()
}

pub fn get_logged_in_client(username: &str, email: &str, role: &str) -> Client {
    let password = "1234";
    let output = create_test_user(username, email, password, role);

    println!("{:?}", output);

    let client = Client::new();
    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": username,
            "email": email,
            "password": password
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    let auth_header = format!("Bearer {}", json["token"].as_str().unwrap());

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&auth_header).unwrap(),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub fn get_client_with_logged_in_viewer() -> Client {
    get_logged_in_client("test_viewer", "test_viewer@gmail.com", "viewer")
}

pub fn get_client_with_logged_in_editor() -> Client {
    get_logged_in_client("test_editor", "test_editor@gmail.com", "editor")
}
