use std::str::FromStr;

use diesel::{Connection, PgConnection};

use crate::{
    auth,
    models::{NewUser, RoleCode},
    repositories::{RoleRepository, UserRepository},
};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Unable to read database URL from env");
    PgConnection::establish(&database_url).expect("Unable to connect to the database")
}

pub fn create_user(username: String, email: String, password: String, role_codes: Vec<String>) {
    let mut connection = load_db_connection();

    let password_hash = auth::hash_password(password).unwrap();
    let new_user = NewUser {
        username,
        email,
        password: password_hash.to_string(),
    };

    let role_codes = role_codes
        .iter()
        .map(|v| RoleCode::from_str(&v).unwrap())
        .collect();

    let user = UserRepository::create(&mut connection, new_user, role_codes).unwrap();
    println!("User created: {:?}", user);

    let roles = RoleRepository::find_by_user(&mut connection, &user).unwrap();
    for role in roles {
        println!("Role assigned: {:?}", role);
    }
}

pub fn list_users() {
    let mut connection = load_db_connection();

    let users = UserRepository::find_with_roles(&mut connection).unwrap();

    for user in users {
        println!("User: {:?}", user);
        println!("Roles:");
        for role in user.1.iter() {
            println!("\t{:?}", role.1);
        }
    }
}

pub fn delete_user(id: i32) {
    let mut connection = load_db_connection();

    UserRepository::delete(&mut connection, id).unwrap();
}
