use std::str::FromStr;

use diesel::{Connection, PgConnection};

use crate::{
    auth,
    models::{NewCompany, NewUser, RoleCode, User, UserType},
    repositories::{CompanyRepository, RoleRepository, UserRepository},
};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Unable to read database URL from env");
    PgConnection::establish(&database_url).expect("Unable to connect to the database")
}

pub fn create_user(
    username: String,
    email: String,
    password: String,
    confirmed: bool,
    user_type_code: &str,
    role_codes: Vec<String>,
) {
    let mut connection = load_db_connection();

    let password_hash = auth::hash_password(password).unwrap();
    let new_user = NewUser {
        username,
        email,
        password: password_hash.to_string(),
    };

    let role_codes = role_codes
        .iter()
        .map(|v| RoleCode::from_str(v).unwrap())
        .collect();

    let user = UserRepository::create(&mut connection, new_user, role_codes).unwrap();

    if confirmed {
        let _ = UserRepository::confirm_signup(&mut connection, user.id);
    }

    let user_type: UserType = FromStr::from_str(user_type_code).unwrap();
    let user = UserRepository::set_user_type(&mut connection, user.id, &user_type).unwrap();

    let roles = RoleRepository::find_by_user(&mut connection, &user).unwrap();

    println!(
        "User created: {:?}",
        User {
            confirmed,
            user_type,
            ..user
        }
    );
    for role in roles {
        println!("Roles assigned: {:?}", role);
    }
}

pub fn list_users() {
    let mut connection = load_db_connection();

    let users = UserRepository::find_with_roles(&mut connection).unwrap();

    for user in users {
        let companies = UserRepository::find_companies(&mut connection, user.0.id).unwrap();

        println!("User: {:?}", user);

        if !companies.is_empty() {
            println!("Companies:");
            for company in companies {
                println!("\t{}", company.name);
            }
        }

        println!("Roles:");
        for role in user.1.iter() {
            println!("\t{:?}", role);
        }
    }
}

pub fn delete_user(id: i32) {
    let mut connection = load_db_connection();

    UserRepository::delete(&mut connection, id).unwrap();
}

pub fn set_user_type(id: i32, user_type_code: &str) {
    let mut connection = load_db_connection();

    let user_type: UserType = FromStr::from_str(user_type_code).unwrap();
    let user = UserRepository::set_user_type(&mut connection, id, &user_type).unwrap();

    println!(
        "User type of user: {:?} set to: {:?}",
        user.username, user.user_type
    );
}

pub fn set_roles(id: i32, role_codes: Vec<String>, is_adding: bool) {
    let mut connection = load_db_connection();

    let role_codes = role_codes
        .iter()
        .map(|v| RoleCode::from_str(v).expect("Invalid role code"))
        .collect::<Vec<RoleCode>>();

    let user = UserRepository::find(&mut connection, id).unwrap();

    if is_adding {
        UserRepository::add_roles(&mut connection, &user, &role_codes).unwrap();
    } else {
        UserRepository::remove_roles(&mut connection, &user, &role_codes).unwrap();
    }

    let roles = RoleRepository::find_by_user(&mut connection, &user).unwrap();

    println!("User roles updated for user: {:?}", user.username);

    for role in roles {
        println!("Roles assigned: {:?}", role);
    }
}

pub fn create_company(
    name: String,
    email: Option<String>,
    website: Option<String>,
    address: Option<String>,
) {
    let mut connection = load_db_connection();

    let company = NewCompany {
        name,
        email,
        website,
        address,
    };

    let company = CompanyRepository::create(&mut connection, company).unwrap();

    println!("Company created: {:?}", company);
}

pub fn add_user_to_company(company_name: String, user_email: String, role_codes: Vec<String>) {
    let mut connection = load_db_connection();

    let role_codes: Vec<RoleCode> = role_codes
        .iter()
        .map(|v| RoleCode::from_str(v).unwrap())
        .collect();

    let company = CompanyRepository::find_by_name(&mut connection, company_name.as_str())
        .expect("Company not found");
    let user = UserRepository::find_by_email(&mut connection, user_email.as_str()).unwrap();

    if user.user_type != UserType::Enterprise {
        panic!("User {} is not an enterprise user.", user.username);
    }

    CompanyRepository::add_user(
        &mut connection,
        company.clone(),
        user.clone(),
        role_codes.clone(),
    )
    .unwrap();

    println!(
        "User added: company:{}, username:{}, roles:{:?}",
        company.name, user.username, role_codes
    );
}

pub fn list_companies() {
    let mut connection = load_db_connection();

    let companies = CompanyRepository::list(&mut connection).unwrap();

    for company in companies {
        println!("Company: {:?}", company);
    }
}

pub fn delete_company(id: i32) {
    let mut connection = load_db_connection();

    CompanyRepository::delete(&mut connection, id).unwrap();
}
