# Rust Template CLI

The `Rust Template CLI` is a command-line interface designed to manage users and companies within a system. The CLI is executed using Cargo, and it offers various commands and subcommands for different tasks.

To run the CLI, use the following command:

```bash
docker compose exec app cargo run --bin cli [COMMAND] [SUBCOMMAND] [OPTIONS]
```

## Overview

This CLI tool allows administrators to perform key actions related to user and company management. It is structured with two primary commands:
1. **Users Management**: Creating, listing, deleting, and modifying users and their roles.
2. **Companies Management**: Creating, listing, deleting companies, and managing the users associated with these companies.

## Commands and Subcommands

### 1. Users Management

The `users` command deals with user-related operations. Here’s how it works:

#### Creating a User

The `create` subcommand is used to add a new user to the system. The user can be assigned multiple roles, and you can specify whether their registration is confirmed via email.

```bash
docker compose exec app cargo run --bin cli users create --username <USERNAME> --email <EMAIL> --password <PASSWORD> [OPTIONS]
```

- The `username`, `email`, and `password` are mandatory fields.
- The `confirmed` option allows you to specify if the user's registration is verified (default: `false`).
- The `type` option indicates whether the user is a regular user or belongs to an enterprise (default: `regular`).
- The `roles` option allows you to assign one or more roles to the user (default: `viewer`).

**Important Notes:**

- **System Admin vs. Enterprise Admin**: 
  - A regular user with the Admin role is considered a **system admin** and has administrative privileges across the entire system.
  - An enterprise user with the Admin role is considered an **enterprise admin** and has administrative privileges only within the scope of their associated company.

**Example:**
If you want to create a new user named John with administrative privileges, you might use:

```bash
docker compose exec app cargo run --bin cli users create --username john_doe --email john@example.com --password secret123 --confirmed true --roles admin
```

This command creates a confirmed user with administrative rights.

#### Listing Users

The `list` subcommand lists all users currently in the system.

```bash
docker compose exec app cargo run --bin cli users list
```

- The command queries the system and retrieves a list of all registered users, displaying their usernames, email addresses, roles, and other details.

**Example:**
To quickly check all users in the system, you would simply run:

```bash
docker compose exec app cargo run --bin cli users list
```

#### Deleting a User

The `delete` subcommand removes a user from the system by their ID.

```bash
docker compose exec app cargo run --bin cli users delete <USER_ID>
```

- The command looks up the user by their ID and permanently deletes their record from the system.

**Example:**
To remove a user with ID 42, you would execute:

```bash
docker compose exec app cargo run --bin cli users delete 42
```

#### Setting User Type

The `set_type` subcommand changes the type of an existing user.

```bash
docker compose exec app cargo run --bin cli users set_type <USER_ID> <USER_TYPE>
```

- This command allows you to modify a user’s classification (e.g., from `regular` to `enterprise`).

**Important Note:**

- **Regular vs. Enterprise User**: 
  - A **Regular** user cannot be added to a company. You must first change the user’s type to **Enterprise** using this command before associating them with a company.

**Example:**
To upgrade a user to an enterprise account:

```bash
docker compose exec app cargo run --bin cli users set_type 42 enterprise
```

#### Adding Roles to a User

The `add_roles` subcommand adds one or more roles to an existing user.

```bash
docker compose exec app cargo run --bin cli users add_roles <USER_ID> <ROLES>
```

- The command updates the user's roles by appending new ones without removing the existing roles.

**Example:**
To add editor and admin roles to a user:

```bash
docker compose exec app cargo run --bin cli users add_roles 42 editor,admin
```

#### Removing Roles from a User

The `remove_roles` subcommand removes specific roles from an existing user.

```bash
docker compose exec app cargo run --bin cli users remove_roles <USER_ID> <ROLES>
```

- The command modifies the user’s roles by removing the specified roles while keeping the others intact.

**Example:**
To remove the admin role from a user:

```bash
docker compose exec app cargo run --bin cli users remove_roles 42 admin
```

### 2. Companies Management

The `companies` command deals with company-related operations. Here’s how it works:

#### Creating a Company

The `create` subcommand is used to register a new company in the system.

```bash
docker compose exec app cargo run --bin cli companies create --name <COMPANY_NAME> [OPTIONS]
```

- The `name` field is mandatory, while `email`, `website`, and `address` are optional.
- This command registers the company in the system, storing its details for future reference.

**Example:**
To create a company called "Acme Corp":

```bash
docker compose exec app cargo run --bin cli companies create --name "Acme Corp" --email info@acme.com --website acme.com --address "123 Acme Way"
```

#### Listing Companies

The `list` subcommand displays all companies in the system.

```bash
docker compose exec app cargo run --bin cli companies list
```

- The command retrieves and displays a list of all registered companies.

**Example:**
To get a full list of companies, you would simply run:

```bash
docker compose exec app cargo run --bin cli companies list
```

#### Deleting a Company

The `delete` subcommand removes a company from the system by its ID.

```bash
docker compose exec app cargo run --bin cli companies delete <COMPANY_ID>
```

- The command permanently removes the specified company from the database.

**Example:**
To delete a company with ID 5:

```bash
docker compose exec app cargo run --bin cli companies delete 5
```

#### Adding a User to a Company

The `add` subcommand is used to assign a user to a company, potentially with specific roles.

```bash
docker compose exec app cargo run --bin cli companies add --name <COMPANY_NAME> --email <USER_EMAIL> [OPTIONS]
```

- You specify the company name and user’s email, and optionally assign roles that the user will have within that company.

**Important Notes:**
- You can only add **Enterprise** users to a company. If the user is currently a **Regular** user, you must first change their type using the [`set_type`](#setting-user-type) subcommand.
- The roles specified in this command will replace the user's existing roles.

**Example:**
To add a user with email `john@example.com` to "Acme Corp" as an admin:

```bash
docker compose exec app cargo run --bin cli companies add --name "Acme Corp" --email john@example.com --roles admin
```

This command will only succeed if `john@example.com` is an **Enterprise** user.