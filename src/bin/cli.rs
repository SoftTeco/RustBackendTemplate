use clap::{Arg, Command};

extern crate rust_template;

const CMD_USERS: &str = "users";
const CMD_CREATE: &str = "create";
const CMD_LIST: &str = "list";
const CMD_DELETE: &str = "delete";
const CMD_COMPANIES: &str = "companies";
const CMD_ADD: &str = "add";
const ARG_USERNAME: &str = "username";
const ARG_EMAIL: &str = "email";
const ARG_PASSWORD: &str = "password";
const ARG_CONFIRMED: &str = "confirmed";
const ARG_ROLES: &str = "roles";
const ARG_ID: &str = "id";
const ARG_NAME: &str = "name";
const ARG_WEBSITE: &str = "website";
const ARG_ADDRESS: &str = "address";
const ARG_TYPE: &str = "type";

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
                        .arg(
                            Arg::new(ARG_USERNAME)
                                .long(ARG_USERNAME)
                                .short('u')
                                .required(true),
                        )
                        .arg(
                            Arg::new(ARG_EMAIL)
                                .long(ARG_EMAIL)
                                .short('e')
                                .required(true),
                        )
                        .arg(
                            Arg::new(ARG_PASSWORD)
                                .long(ARG_PASSWORD)
                                .short('p')
                                .required(true),
                        )
                        .arg(
                            Arg::new(ARG_CONFIRMED)
                                .long(ARG_CONFIRMED)
                                .short('c')
                                .default_value("false"),
                        )
                        .arg(
                            Arg::new(ARG_TYPE)
                                .long(ARG_TYPE)
                                .short('t')
                                .default_value("regular"),
                        )
                        .arg(
                            Arg::new(ARG_ROLES)
                                .long(ARG_ROLES)
                                .short('r')
                                .default_value("viewer")
                                .num_args(1..)
                                .value_delimiter(','),
                        ),
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
        .subcommand(
            Command::new(CMD_COMPANIES)
                .about("Rust Template company management CLI")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new(CMD_CREATE)
                        .about("Creating a company")
                        .arg_required_else_help(true)
                        .arg(Arg::new(ARG_NAME).long(ARG_NAME).short('n').required(true))
                        .arg(Arg::new(ARG_EMAIL).long(ARG_EMAIL).short('e'))
                        .arg(Arg::new(ARG_WEBSITE).long(ARG_WEBSITE).short('w'))
                        .arg(Arg::new(ARG_ADDRESS).long(ARG_ADDRESS).short('a')),
                )
                .subcommand(Command::new(CMD_LIST).about("List existing companies"))
                .subcommand(
                    Command::new(CMD_DELETE).about("Delete company by ID").arg(
                        Arg::new(ARG_ID)
                            .required(true)
                            .value_parser(clap::value_parser!(i32)),
                    ),
                )
                .subcommand(
                    Command::new(CMD_ADD)
                        .about("Adding a user to a company")
                        .arg_required_else_help(true)
                        .arg(Arg::new(ARG_NAME).long(ARG_NAME).short('n').required(true))
                        .arg(
                            Arg::new(ARG_EMAIL)
                                .long(ARG_EMAIL)
                                .short('e')
                                .required(true),
                        )
                        .arg(
                            Arg::new(ARG_ROLES)
                                .long(ARG_ROLES)
                                .short('r')
                                .default_value("viewer")
                                .num_args(1..)
                                .value_delimiter(','),
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
                    .get_one::<String>(ARG_CONFIRMED)
                    .map(|v| v.parse::<bool>().unwrap())
                    .unwrap()
                    .to_owned(),
                sub_matches
                    .get_one::<String>(ARG_TYPE)
                    .map(|v| v.as_str())
                    .unwrap(),
                sub_matches
                    .get_many::<String>(ARG_ROLES)
                    .unwrap()
                    .map(|v| v.to_string())
                    .collect(),
            ),
            Some((CMD_LIST, _)) => rust_template::commands::list_users(),
            Some((CMD_DELETE, sub_matches)) => rust_template::commands::delete_user(
                sub_matches.get_one::<i32>(ARG_ID).unwrap().to_owned(),
            ),
            _ => {}
        },
        Some((CMD_COMPANIES, sub_matches)) => match sub_matches.subcommand() {
            Some((CMD_CREATE, sub_matches)) => rust_template::commands::create_company(
                sub_matches.get_one::<String>(ARG_NAME).unwrap().to_owned(),
                sub_matches
                    .get_one::<String>(ARG_EMAIL)
                    .map(|v| v.to_string()),
                sub_matches
                    .get_one::<String>(ARG_WEBSITE)
                    .map(|v| v.to_string()),
                sub_matches
                    .get_one::<String>(ARG_ADDRESS)
                    .map(|v| v.to_string()),
            ),
            Some((CMD_LIST, _)) => rust_template::commands::list_companies(),
            Some((CMD_DELETE, sub_matches)) => rust_template::commands::delete_company(
                sub_matches.get_one::<i32>(ARG_ID).unwrap().to_owned(),
            ),
            Some((CMD_ADD, sub_matches)) => rust_template::commands::add_user_to_company(
                sub_matches.get_one::<String>(ARG_NAME).unwrap().to_owned(),
                sub_matches.get_one::<String>(ARG_EMAIL).unwrap().to_owned(),
                sub_matches
                    .get_many::<String>(ARG_ROLES)
                    .unwrap()
                    .map(|v| v.to_string())
                    .collect(),
            ),
            _ => {}
        },
        _ => {}
    }
}
