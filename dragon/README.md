# DragonFruit API

A secure credential management REST API built with Rust.

## Features

- User authentication with JWT tokens
- Optional TOTP-based two-factor authentication
- Hierarchical category system with support for nested folders
- Secure password storage with encryption at rest
- PostgreSQL database with SQLx for type-safe queries

## Tech Stack

- Rust 1.75+
- Axum web framework
- SQLx with PostgreSQL
- Argon2 for password hashing
- JWT for authentication tokens
- TOTP for optional 2FA

## Setup

1. Install Rust and Cargo
2. Install PostgreSQL and create a database
3. Clone this repository
4. Create a `.env` file with:

```
DATABASE_URL=postgres://username:password@localhost/dragonfruit_db
JWT_SECRET=your_secure_random_secret
FRONTEND_ORIGIN=http://localhost:5173
```

5. Run the application:

```bash
cargo run
```

The server will start on http://localhost:3000.

## API Endpoints

### Authentication

- `POST /api/register` - Register a new user
- `POST /api/login` - Log in with email and password
- `GET /api/me` - Get current user info
- `POST /api/totp/setup` - Set up TOTP 2FA
- `POST /api/totp/verify` - Verify and enable TOTP
- `POST /api/totp/disable` - Disable TOTP

### Categories

- `GET /api/categories` - Get all categories (nested tree)
- `POST /api/categories` - Create a new category
- `PUT /api/categories/:id` - Update a category
- `DELETE /api/categories/:id` - Delete a category

### Credentials

- `GET /api/credentials` - Get all credentials
- `POST /api/credentials` - Create a new credential
- `GET /api/credentials/:id` - Get a credential with password
- `PUT /api/credentials/:id` - Update a credential
- `DELETE /api/credentials/:id` - Delete a credential
- `GET /api/categories/:id/credentials` - Get credentials by category

## Security

The API implements several security measures:

- Password hashing with Argon2id
- JWT tokens for authentication
- Password encryption at rest
- Optional TOTP-based 2FA
- CORS protection for frontend access

## Development

To run the development server with hot reloading:

```bash
cargo install cargo-watch
cargo watch -x run
``` 