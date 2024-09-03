use rand::distributions::Alphanumeric;
use rand::Rng;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::StatusCode;
use serde_json::{json, Value};
use std::process::{Command, Output};

pub const APP_HOST: &'static str = "http://127.0.0.1:8000";
pub const SESSION_ID_LENGTH: usize = 128;

pub fn create_test_user(
    username: &str,
    email: &str,
    password: &str,
    roles: &str,
    is_confirmed: &str,
) -> Output {
    Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("-u")
        .arg(username)
        .arg("-e")
        .arg(email)
        .arg("-p")
        .arg(password)
        .arg("-c")
        .arg(is_confirmed)
        .arg("-r")
        .arg(roles)
        .output()
        .unwrap()
}

pub fn delete_test_user(create_output: Output) {
    let create_stdout = String::from_utf8(create_output.stdout).unwrap();

    let prefix = "User created: User { id: ";
    let suffix = ", username:";
    let start_bytes = create_stdout.find(prefix).unwrap_or(0) + prefix.len();
    let end_bytes = create_stdout.find(suffix).unwrap_or(create_stdout.len());

    let user_id = &create_stdout[start_bytes..end_bytes];
    println!("Delete test user:{}", user_id);

    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("delete")
        .arg(user_id)
        .status();
}

pub fn get_logged_in_client(username: &str, email: &str, role: &str) -> (Client, Output) {
    let password = "123456aA";
    let output = create_test_user(username, email, password, role, &true.to_string());

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

    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    return (client, output);
}

pub fn get_client_with_logged_in_viewer() -> (Client, Output) {
    let username = format!("testViewer{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    get_logged_in_client(username.as_str(), email.as_str(), "viewer")
}

pub fn get_client_with_logged_in_editor() -> (Client, Output) {
    let username = format!("testEditor{}", rand::random::<u32>());
    let email = format!("{}@gmail.com", username);
    get_logged_in_client(username.as_str(), email.as_str(), "editor")
}

pub fn generate_test_token(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
