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

- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - Log in with email and password
- `GET /api/auth/profile` - Get current user info
- `PUT /api/auth/profile` - Update user profile
- `POST /api/auth/totp/generate` - Set up TOTP 2FA
- `POST /api/auth/totp/enable` - Verify and enable TOTP

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

## Data Models

### User

```
{
  "id": "uuid",
  "email": "string",
  "totp_enabled": boolean,
  "created_at": "timestamp",
  "updated_at": "timestamp"
}
```

### Category

```
{
  "id": "uuid",
  "name": "string",
  "parent_id": "uuid or null",
  "created_at": "timestamp",
  "updated_at": "timestamp",
  "children": [Category]  // For nested structure
}
```

### Credential

```
{
  "id": "uuid",
  "name": "string",
  "category_id": "uuid or null",
  "website": "string or null",
  "username": "string or null",
  "password": "string",  // Only included in single credential response
  "notes": "string or null",
  "created_at": "timestamp",
  "updated_at": "timestamp"
}
```

## API Structure Diagram

```
DragonFruit API
│
├── Authentication
│   ├── POST /api/auth/register - Register new user
│   ├── POST /api/auth/login - Login and get JWT token
│   ├── GET /api/auth/profile - Get user profile
│   ├── PUT /api/auth/profile - Update user profile
│   ├── POST /api/auth/totp/generate - Generate TOTP secret
│   └── POST /api/auth/totp/enable - Enable TOTP 2FA
│
├── Categories (requires authentication)
│   ├── GET /api/categories - Get all categories as tree
│   ├── POST /api/categories - Create category
│   ├── PUT /api/categories/:id - Update category
│   └── DELETE /api/categories/:id - Delete category
│
└── Credentials (requires authentication)
    ├── GET /api/credentials - Get all credentials
    ├── POST /api/credentials - Create credential
    ├── GET /api/credentials/:id - Get credential with password
    ├── PUT /api/credentials/:id - Update credential
    ├── DELETE /api/credentials/:id - Delete credential
    └── GET /api/categories/:id/credentials - Get credentials by category
```

## Database Schema

```
┌─────────────┐       ┌─────────────┐       ┌─────────────┐
│    users    │       │  categories │       │ credentials │
├─────────────┤       ├─────────────┤       ├─────────────┤
│ id          │       │ id          │       │ id          │
│ email       │       │ user_id     │───┐   │ user_id     │───┐
│password_hash│       │ name        │   │   │ category_id │╌╌╌┘
│ totp_secret │       │ parent_id   │╌╌╌┘   │ name        │
│ totp_enabled│       │ created_at  │       │ website     │
│ created_at  │       │ updated_at  │       │ username    │
│ updated_at  │       └─────────────┘       │ password_encrypted│
└─────────────┘                             │ notes       │
                                            │ created_at  │
                                            │ updated_at  │
                                            └─────────────┘
```

## Authentication Flow

```
1. Register User
   Client ──POST /api/auth/register──> Server
           <──201 Created + User Data──

2. Login
   Client ──POST /api/auth/login──> Server
           <──200 OK + JWT Token──

3. Subsequent Requests
   Client ──Request + Authorization Header──> Server
           <──Response──

4. 2FA Setup (Optional)
   Client ──POST /api/auth/totp/generate──> Server
           <──200 OK + TOTP Secret + QR Code URL──
   
   Client ──POST /api/auth/totp/enable + TOTP Code──> Server
           <──200 OK──
   
   Client ──Login with 2FA──> Server
           <──200 OK + JWT Token──
```

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
```
