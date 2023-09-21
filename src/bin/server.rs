use rocket_db_pools::Database;

extern crate rust_template;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![rust_template::rocket_routes::authorization::login,],
        )
        .attach(rust_template::rocket_routes::DbConnection::fairing())
        .attach(rust_template::rocket_routes::CacheConnection::init())
        .launch()
        .await;
}
