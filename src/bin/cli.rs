use clap::{Arg, Command};

extern crate rust_template;

const CMD_USERS: &str = "users";
const CMD_CREATE: &str = "create";
const CMD_LIST: &str = "list";
const CMD_DELETE: &str = "delete";
const ARG_USERNAME: &str = "username";
const ARG_EMAIL: &str = "email";
const ARG_PASSWORD: &str = "password";
const ARG_CONFIRMED: &str = "confirmed";
const ARG_ROLES: &str = "roles";
const ARG_ID: &str = "id";

fn main() {
    let matches = Command::new("Rust Template")
        .about("Rust Template CLI")
        .arg_required_else_help(true)
        .subcommand(
            Command::new(CMD_USERS)
                .about("Rust Template user management CLI")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new(CMD_CREATE)
                        .about("Creating a user with multiple roles assigned")
                        .arg_required_else_help(true)
                        .arg(Arg::new(ARG_USERNAME).required(true))
                        .arg(Arg::new(ARG_EMAIL).required(true))
                        .arg(Arg::new(ARG_PASSWORD).required(true))
                        .arg(
                            Arg::new(ARG_ROLES)
                                .required(true)
                                .num_args(1..)
                                .value_delimiter(','),
                        )
                        .arg(Arg::new(ARG_CONFIRMED).required(true)),
                )
                .subcommand(Command::new(CMD_LIST).about("List existing users"))
                .subcommand(
                    Command::new(CMD_DELETE).about("Delete user by ID").arg(
                        Arg::new(ARG_ID)
                            .required(true)
                            .value_parser(clap::value_parser!(i32)),
                    ),
                ),
        )
        .get_matches();

    #[allow(clippy::single_match)]
    match matches.subcommand() {
        Some((CMD_USERS, sub_matches)) => match sub_matches.subcommand() {
            Some((CMD_CREATE, sub_matches)) => rust_template::commands::create_user(
                sub_matches
                    .get_one::<String>(ARG_USERNAME)
                    .unwrap()
                    .to_owned(),
                sub_matches.get_one::<String>(ARG_EMAIL).unwrap().to_owned(),
                sub_matches
                    .get_one::<String>(ARG_PASSWORD)
                    .unwrap()
                    .to_owned(),
                sub_matches
                    .get_many::<String>(ARG_ROLES)
                    .unwrap()
                    .map(|v| v.to_string())
                    .collect(),
                sub_matches
                    .get_one::<String>(ARG_CONFIRMED)
                    .map(|v| v.parse::<bool>().unwrap())
                    .unwrap()
                    .to_owned(),
            ),
            Some((CMD_LIST, _)) => rust_template::commands::list_users(),
            Some((CMD_DELETE, sub_matches)) => rust_template::commands::delete_user(
                sub_matches.get_one::<i32>(ARG_ID).unwrap().to_owned(),
            ),
            _ => {}
        },
        _ => {}
    }
}
