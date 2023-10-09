use rocket_db_pools::Database;

extern crate rust_template;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                rust_template::rocket_routes::authorization::login,
                rust_template::rocket_routes::authorization::signup,
                rust_template::rocket_routes::authorization::reset_password,
                rust_template::rocket_routes::authorization::change_password,
                rust_template::rocket_routes::profile::me,
                rust_template::rocket_routes::profile::update_password,
            ],
        )
        .attach(rust_template::rocket_routes::DbConnection::fairing())
        .attach(rust_template::rocket_routes::CacheConnection::init())
        .launch()
        .await;
}
