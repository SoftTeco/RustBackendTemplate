use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::fairing::AdHoc;

use rocket::{Build, Rocket};
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;
use rust_template::rocket_routes::{authorization, profile, Cors};
use rust_template::rocket_routes::{CacheConnection, DbConnection};
use rust_template::{dto, errors};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::openapi::{ContactBuilder, InfoBuilder, LicenseBuilder, OpenApiBuilder, ServerBuilder};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

extern crate rust_template;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[rocket::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            authorization::login,
            authorization::signup,
            authorization::reset_password,
            authorization::change_password,
            authorization::confirm_signup,
            profile::me,
            profile::update_password,
            profile::update_user,
        ),
        components(schemas(
            dto::UserProfileDto,
            dto::CredentialsDto,
            dto::AuthTokenDto,
            dto::NewPasswordDto,
            dto::NewUserDto,
            dto::NewUserResponseDto,
            dto::ResetPasswordEmailDto,
            dto::UpdateUserDto,
            errors::AuthError,
            errors::ProfileError,
        )),
        modifiers(&SecurityAddon),
    )]
    struct ApiDoc;

    let openapi = set_openapi_doc_parameters(ApiDoc::openapi().into()).build();

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();
            components.add_security_scheme(
                "token",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            );
        }
    }

    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                rust_template::rocket_routes::options,
                authorization::login,
                authorization::signup,
                authorization::reset_password,
                authorization::change_password,
                authorization::confirm_signup,
                profile::me,
                profile::update_password,
                profile::update_user,
            ],
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", openapi),
        )
        .attach(Cors)
        .attach(DbConnection::fairing())
        .attach(CacheConnection::init())
        .attach(Template::fairing())
        .attach(AdHoc::on_ignite(
            "Run database migrations",
            run_db_migrations,
        ))
        .launch()
        .await;
}

fn set_openapi_doc_parameters(builder: OpenApiBuilder) -> OpenApiBuilder {
    let info = InfoBuilder::new()
        .title("Template API")
        .version(VERSION)
        .description(Some("API for Template mobile app"))
        .contact(Some(
            ContactBuilder::new()
                .name(Some("Anton Savich"))
                .email(Some("pcfaktor@gmail.com"))
                .url(Some("https://github.com/SoftTeco"))
                .build(),
        ))
        .license(Some(
            LicenseBuilder::new()
                .name("MIT license")
                .url(Some("https://opensource.org/licenses/MIT"))
                .build(),
        ))
        .build();

    let base_url = std::env::var("BASE_URL").expect("Unable to read base URL from env");

    let server = ServerBuilder::new()
        .url(base_url)
        .description(Some("The URL of the server in the Dev environment"))
        .build();

    builder.info(info).servers(Some(vec![server].into_iter()))
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    let db = DbConnection::get_one(&rocket)
        .await
        .expect("Cannot connect to postgres");
    db.run(move |connection| {
        connection.run_pending_migrations(MIGRATIONS).unwrap();
    })
    .await;
    rocket
}
