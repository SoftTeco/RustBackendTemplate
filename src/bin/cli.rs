use clap::{Arg, Command};

extern crate rust_template;

const CMD_USERS: &str = "users";
const CMD_CREATE: &str = "create";
const CMD_LIST: &str = "list";
const CMD_DELETE: &str = "delete";
const CMD_COMPANIES: &str = "companies";
const CMD_ADD: &str = "add";
const CMD_SET_TYPE: &str = "set_type";
const CMD_ADD_ROLES: &str = "add_roles";
const CMD_REMOVE_ROLES: &str = "remove_roles";
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
                                .help("Username of the user")
                                .required(true),
                        )
                        .arg(
                            Arg::new(ARG_EMAIL)
                                .long(ARG_EMAIL)
                                .short('e')
                                .help("Email of the user")
                                .required(true),
                        )
                        .arg(
                            Arg::new(ARG_PASSWORD)
                                .long(ARG_PASSWORD)
                                .short('p')
                                .help("Password of the user")
                                .required(true),
                        )
                        .arg(
                            Arg::new(ARG_CONFIRMED)
                                .long(ARG_CONFIRMED)
                                .short('c')
                                .help("Is the user's registration confirmed by e-mail")
                                .default_value("false"),
                        )
                        .arg(
                            Arg::new(ARG_TYPE)
                                .long(ARG_TYPE)
                                .short('t')
                                .help("Type of the user (regular or enterprise). ")
                                .default_value("regular"),
                        )
                        .arg(
                            Arg::new(ARG_ROLES)
                                .long(ARG_ROLES)
                                .short('r')
                                .help(
                                    "Required roles for the user (viewer, editor, admin).
Multiple roles can be separated by comma."
                                )
                                .default_value("viewer")
                                .num_args(1..)
                                .value_delimiter(','),
                        ),
                )
                .subcommand(Command::new(CMD_LIST).about("List existing users"))
                .subcommand(
                    Command::new(CMD_DELETE).about("Delete user by ID")
                    .arg_required_else_help(true)
                    .arg(
                        Arg::new(ARG_ID)
                            .required(true)
                            .help("ID of the user to delete")
                            .value_parser(clap::value_parser!(i32)),
                    ),
                )
                .subcommand(
                    Command::new(CMD_SET_TYPE)
                        .about("Set user type")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new(ARG_ID)
                                .required(true)
                                .help("ID of the user to set type")
                                .value_parser(clap::value_parser!(i32)),
                        )
                        .arg(Arg::new(ARG_TYPE).required(true)
                            .help("Type of the user (regular or enterprise)")
                        ),
                )
                .subcommand(
                    Command::new(CMD_ADD_ROLES)
                        .about("Add roles to user")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new(ARG_ID)
                                .required(true)
                                .help("ID of the user to add roles to")
                                .value_parser(clap::value_parser!(i32)),
                        )
                        .arg(
                            Arg::new(ARG_ROLES)
                                .required(true)
                                .help("Roles to add to the user. Multiple roles can be separated by comma.")
                                .num_args(1..)
                                .value_delimiter(','),
                        ),
                )
                .subcommand(
                    Command::new(CMD_REMOVE_ROLES)
                        .about("Remove roles from user")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new(ARG_ID)
                                .required(true)
                                .help("ID of the user to remove roles from")
                                .value_parser(clap::value_parser!(i32)),
                        )
                        .arg(
                            Arg::new(ARG_ROLES)
                                .required(true)
                                .help("Roles to remove from the user. Multiple roles can be separated by comma.")
                                .num_args(1..)
                                .value_delimiter(','),
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
                        .arg(Arg::new(ARG_NAME).long(ARG_NAME).short('n').required(true).help("Company name"))
                        .arg(Arg::new(ARG_EMAIL).long(ARG_EMAIL).short('e').help("Company email"))
                        .arg(Arg::new(ARG_WEBSITE).long(ARG_WEBSITE).short('w').help("Company website"))
                        .arg(Arg::new(ARG_ADDRESS).long(ARG_ADDRESS).short('a').help("Company address")),
                )
                .subcommand(Command::new(CMD_LIST).about("List existing companies"))
                .subcommand(
                    Command::new(CMD_DELETE).about("Delete company by ID")
                    .arg_required_else_help(true)
                    .arg(
                        Arg::new(ARG_ID)
                            .required(true)
                            .help("ID of the company to delete")
                            .value_parser(clap::value_parser!(i32)),
                    ),
                )
                .subcommand(
                    Command::new(CMD_ADD)
                        .about("Adding a user to a company")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new(ARG_NAME)
                                .long(ARG_NAME)
                                .short('n')
                                .help("Company name to add user to")
                                .required(true),
                        )
                        .arg(
                            Arg::new(ARG_EMAIL)
                                .long(ARG_EMAIL)
                                .short('e')
                                .help("Email of the user to be added to the company")
                                .required(true),
                        )
                        .arg(
                            Arg::new(ARG_ROLES)
                                .long(ARG_ROLES)
                                .short('r')
                                .help(
                                    "Required roles for the user (viewer, editor, admin).
Multiple roles can be separated by comma. Existing roles will be replaced.",
                                )
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
            Some((CMD_SET_TYPE, sub_matches)) => rust_template::commands::set_user_type(
                sub_matches.get_one::<i32>(ARG_ID).unwrap().to_owned(),
                sub_matches
                    .get_one::<String>(ARG_TYPE)
                    .map(|v| v.as_str())
                    .unwrap(),
            ),
            Some((CMD_ADD_ROLES, sub_matches)) => rust_template::commands::set_roles(
                sub_matches.get_one::<i32>(ARG_ID).unwrap().to_owned(),
                sub_matches
                    .get_many::<String>(ARG_ROLES)
                    .unwrap()
                    .map(|v| v.to_string())
                    .collect(),
                true,
            ),
            Some((CMD_REMOVE_ROLES, sub_matches)) => rust_template::commands::set_roles(
                sub_matches.get_one::<i32>(ARG_ID).unwrap().to_owned(),
                sub_matches
                    .get_many::<String>(ARG_ROLES)
                    .unwrap()
                    .map(|v| v.to_string())
                    .collect(),
                false,
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
