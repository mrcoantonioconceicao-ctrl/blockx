BLOCKX AUTH SERVICE

Overview

This is the authentication service of the BlockX platform.
It is built in Rust using Axum and follows a clean architecture approach with separation between domain, application, and infrastructure layers.

The service provides secure authentication using JWT access tokens and refresh tokens with rotation and blacklist support.

---

Features

- User registration (email + password)
- Secure password hashing using Argon2
- JWT access token generation (1 hour expiration)
- Refresh token system (7 days expiration)
- Refresh token rotation (anti replay protection)
- Refresh token blacklist (revocation system)
- In-memory user repository (for development)
- Axum-based REST API

---

Architecture

The system follows a layered architecture:

Domain Layer
- User entity and business rules

Application Layer
- Use cases (create_user, login_user, refresh_flow)
- Authentication services (JWT, password hashing, refresh logic)

Infrastructure Layer
- In-memory repositories
- Refresh token store (active + revoked tokens)

State Layer
- Shared application state using Arc

---

Authentication Flow

1. Register
- User sends email and password
- Password is hashed using Argon2
- User is stored in repository

2. Login
- User provides credentials
- System validates user
- JWT access token is generated
- Refresh token is generated and stored

3. Refresh Token Flow
- Client sends refresh token
- System validates token
- Token is checked against blacklist
- Old token is revoked (rotation)
- New access + refresh tokens are generated

---

API Endpoints

POST /auth/register
Request:
{
  "email": "user@example.com",
  "password": "123456"
}

Response:
{
  "user_id": "uuid"
}

---

POST /auth/login
Request:
{
  "email": "user@example.com",
  "password": "123456"
}

Response:
{
  "access_token": "...",
  "refresh_token": "..."
}

---

POST /auth/refresh
Request:
{
  "refresh_token": "..."
}

Response:
{
  "access_token": "...",
  "refresh_token": "..."
}

---

Security Notes

- Passwords are hashed using Argon2
- JWT tokens are signed using HMAC secret
- Refresh tokens are rotated on each use
- Revoked tokens are stored in blacklist
- Tokens have expiration control

---

Future Improvements

- Replace in-memory storage with PostgreSQL or Redis
- Add rate limiting for auth endpoints
- Add device binding for refresh tokens
- Add audit logging (security events)
- Add OAuth2 compatibility layer

---

Project: BlockX
Module: Auth Service
Language: Rust
Framework: Axum
Architecture: Clean Architecture + SOA
