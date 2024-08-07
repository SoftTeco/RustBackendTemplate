<img src="doc/logo.png" alt="icon" width="100"/>

# RustBackendTemplate

[![License: MIT](https://img.shields.io/badge/license-MIT-yellow)](https://opensource.org/licenses/MIT)
![version](https://img.shields.io/badge/dynamic/toml?url=https://raw.githubusercontent.com/SoftTeco/RustBackendTemplate/main/Cargo.toml&query=$.package.version&label=version&color=green)
[![SwaggerDoc](https://img.shields.io/badge/SwaggerDoc-brightgreen)](https://d1bdrzgyyyb9eu.cloudfront.net/swagger-ui/index.html)
[![build](https://github.com/SoftTeco/RustBackendTemplate/actions/workflows/deploy.yml/badge.svg)](https://github.com/SoftTeco/RustBackendTemplate/actions/workflows/deploy.yml)
[![tests](https://github.com/SoftTeco/RustBackendTemplate/actions/workflows/test.yml/badge.svg)](https://github.com/SoftTeco/RustBackendTemplate/actions/workflows/test.yml)
[![lint](https://github.com/SoftTeco/RustBackendTemplate/actions/workflows/lint.yml/badge.svg)](https://github.com/SoftTeco/RustBackendTemplate/actions/workflows/lint.yml)

Backend application for [AndroidAppTemplate](https://github.com/SoftTeco/AndroidAppTemplate)

## Overview

RustBackendTemplate is a backend application that provides a set of APIs for an Android app. It is built using the Rust programming language and leverages the following technologies:

- [Rocket](https://rocket.rs/): A web framework for Rust that provides a simple and intuitive way to build web applications.
- [Diesel](https://diesel.rs/): An ORM (Object-Relational Mapping) library for Rust that simplifies interaction with databases.
- [PostgreSQL](https://www.postgresql.org/): A powerful open-source relational database.
- [Redis](https://redis.io/): An in-memory data structure store used for token management.
- [Docker](https://www.docker.com/): A platform for developing, shipping, and running applications.

## Available Features

- **User Registration and Authentication**: Secure registration and login mechanisms.
- **User Management**:
  - **Create User**: Create new user accounts.
  - **Password management**: Change/restore password.
  - **Delete User**: Delete user accounts via CLI interface.
  - **List Users**: List all users via CLI interface.
- **Profile Management**: Get user profile information.
- **Error Handling and Logging**: Comprehensive error handling and logging throughout the application.
- **Email Sending**: Functionality to send emails for various purposes.

## Command-Line Interface (CLI)

The application includes a CLI interface for managing users. The CLI commands are executed via the following Docker command:

```sh
docker-compose exec app cargo run --bin cli
```

### Available Commands

- **Create User**: 
  ```sh
  docker-compose exec app cargo run --bin cli users create
  ```
- **List Users**: 
  ```sh
  docker-compose exec app cargo run --bin cli users list
  ```
- **Delete User**: 
  ```sh
  docker-compose exec app cargo run --bin cli users delete
  ```

## Getting Started

To get started with RustBackendTemplate, follow these steps:

1. **Clone the repository**:
   ```sh
   git clone https://github.com/SoftTeco/RustBackendTemplate.git
   ```
2. **Navigate to the project directory**:
   ```sh
   cd RustBackendTemplate
   ```
3. **Build the project**:
   ```sh
   docker-compose build
   ```
4. **Start the Docker containers**:
   ```sh
   docker-compose up -d
   ```
5. **Run the application inside the Docker container**:
   ```sh
   docker-compose exec app cargo run
   ```

## Configuration

The application can be configured using environment variables. The following environment variables are available:

- `BASE_URL`: The base URL of the application.
- `POSTGRES_USER`: The PostgreSQL database user.
- `POSTGRES_PASSWORD`: The PostgreSQL database password.
- `POSTGRES_DB`: The PostgreSQL database name.
- `SMTP_HOST`: The SMTP server host for sending emails.
- `SMTP_USERNAME`: The SMTP server username.
- `SMTP_PASSWORD`: The SMTP server password.

## Deployment Process

The deployment process for RustBackendTemplate involves the following components:

- **Docker**: Containers are used to ensure consistency across development, testing, and production environments.
  - **App Container**: Runs the Rust backend application. The Dockerfile for development sets up the development environment with tools like `cargo-watch` and `diesel_cli`. The Dockerfile for deployment creates a lightweight production image.
  - **PostgreSQL Container**: Runs the PostgreSQL database.
  - **Redis Container**: Runs the Redis server for token management.
- **GitHub Actions**: Used for CI/CD processes, including:
  - **Deployment Workflow**: Handles building and deploying the application.
  - **Testing Workflow**: Runs tests to ensure code quality.
  - **Linting Workflow**: Checks code style and potential issues.

### AWS Environment

In production, the application is hosted on AWS with the following components:

- **EC2 Instance**: The application container runs on an EC2 instance.
- **RDS**: PostgreSQL database is managed as an Amazon RDS instance.
- **ElastiCache**: Redis is managed as an Amazon ElastiCache instance.
- **CloudFront**: AWS CloudFront is used to provide TLS/SSL encryption for secure HTTPS access and to enhance performance through caching.

### GitHub Actions Configuration

#### Deployment Workflow (`.github/workflows/deploy.yml`)

This workflow is triggered on push to the `main` branch and handles building and deploying the application. Key steps include:

- **Checkout**: Retrieves the code from the repository.
- **Build Docker Image**: Builds a Docker image using the deployment Dockerfile.
- **Log in to Docker Hub**: Logs in to Docker Hub to push the image.
- **Push Image**: Pushes the built Docker image to Docker Hub.
- **Deploy to Server**: Deploys the Docker container to the server via SSH, pulls the latest image, and starts the container.

#### Testing Workflow (`.github/workflows/test.yml`)

This workflow runs on push and pull requests to the `main` branch. Key steps include:

- **Checkout**: Retrieves the code from the repository.
- **Create .env File**: Creates a `.env` file with environment variables from GitHub secrets.
- **Start Containers**: Starts Docker containers for PostgreSQL, Redis, and the application.
- **Run Migrations**: Runs database migrations.
- **Run App**: Starts the application.
- **Run Tests**: Runs tests.
- **Stop Containers**: Stops and removes Docker containers.

#### Linting Workflow (`.github/workflows/lint.yml`)

This workflow is triggered on push to any branch. Key steps include:

- **Checkout**: Retrieves the code from the repository.
- **Clippy Check**: Runs `cargo clippy` for static code analysis to find potential issues and improve code quality.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

RustBackendTemplate is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.