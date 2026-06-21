# BlockX

<<<<<<< HEAD
**Enterprise Financial Infrastructure Platform built in Rust**

BlockX is a next-generation financial infrastructure and security platform designed for institutions, fintechs, payment providers, and digital asset ecosystems.

The platform is being built from the ground up with a **deterministic core**, emphasizing:

- Security by Design
- Auditability
- Compliance Readiness
- Scalability
- Modular Services
- Cloud-Native Architecture
- High Performance with Rust

---

# Vision

BlockX aims to provide the foundational infrastructure required for:

- Identity and Access Management (IAM)
- Authentication and Authorization
- Digital Wallets
- KYC / KYB
- Financial Ledger
- Payment Gateway Infrastructure
- Risk Management
- Audit and Compliance
- Tokenization of Digital and Real-World Assets (RWA)

The architecture separates deterministic business logic from future intelligent orchestration layers, ensuring predictability, security, and auditability.

---

# Architecture Principles

BlockX follows enterprise engineering principles:

### Security First

Security is treated as a core feature.

### Deterministic Core

Business rules must remain predictable, auditable, and reproducible.

### Auditability

Critical operations must be traceable.

### Service Oriented Architecture (SOA)

Each service evolves independently.

### Clean Architecture

Application, Domain, Infrastructure, and Interface layers are separated.

### Domain Driven Design (DDD)

Business domains are isolated and modeled independently.

### BPMN Ready

Future business workflows will be modeled using BPMN, especially for:

- Onboarding
- KYC
- Risk Analysis
- Payments
- Compliance
- Tokenization
=======
Enterprise Financial Infrastructure Platform built in Rust

BlockX is a next-generation financial infrastructure and security platform designed for institutions, fintechs, payment providers, and digital asset ecosystems.

## Core Principles

- Security by Design
- Deterministic Core
- Auditability
- SOA Architecture
- Clean Architecture
- DDD
- BPMN Ready
- Cloud Native
>>>>>>> 77d1543 (docs(readme): automatic update)

## Current Status

### Sprint 0
- Foundation ✅

<<<<<<< HEAD
## Sprint 0 — Foundation ✅

- Monorepo Structure
- Cargo Workspace
- Shared Crates
- Environment Configuration
- Error Handling
- Observability Foundation

---

## Sprint 1 — Platform Foundation ✅

### Crates

#### Config

Application configuration.

#### Shared

Common utilities.

#### Errors

Centralized error handling.

#### Observability

Logging, tracing and audit foundation.

---

## Sprint 2 — Auth Service 🚀

### Implemented

✅ User Domain Model

✅ User Creation

✅ Email Validation

✅ Password Validation

✅ Argon2id Password Hashing

✅ Password Verification

✅ Login Flow

✅ JWT Token Generation

✅ JWT Token Validation

✅ Duplicate User Protection

✅ User Repository

✅ Find User By Email

✅ Exists User

✅ Configuration through Config crate

✅ Bootstrap Integration

✅ Unit Tests

---

# Authentication Features

## User Creation

Users are created through the application layer.

Features:

- Unique ID generation
- Email validation
- Password validation
- Password hashing
- Duplicate email protection

---

## Password Security

BlockX currently uses:

### Argon2id

Passwords are never stored in plain text.

Example:

```text
$argon2id$v=19$m=19456,t=2,p=1$...
```

Features:

- Random Salt
- Memory Hard
- GPU Resistant
- OWASP Recommended

---

## JWT Authentication

Current implementation:

### Access Token

Features:

- User Identification
- Issued At (iat)
- Expiration Time (exp)
- Signature Validation

### Validation

Implemented:

- Token Decoding
- Signature Verification
- Invalid Token Rejection

---

# Current Repository Pattern

```rust
pub trait UserRepository {

    fn save(
        &self,
        user: &User,
    );

    fn find_by_email(
        &self,
        email: &str,
    ) -> Option<User>;

    fn exists(
        &self,
        email: &str,
    ) -> bool;
}
```

Current implementation:

```text
InMemoryUserRepository
```

Planned:

```text
PostgreSQLUserRepository
```

---

# Current Project Structure

```text
blockx
│
├── apps
│
├── services
│   └── auth
│       ├── api
│       ├── application
│       ├── bootstrap
│       ├── domain
│       └── infrastructure
│
├── crates
│   ├── config
│   ├── errors
│   ├── observability
│   └── shared
│
├── infrastructure
│
├── deployments
│
└── docs
```

---

# Technology Stack

## Language

Rust

---

## Backend

- Axum (planned)
- Tokio

---

## Security

- Argon2id
- JWT
- OWASP Security Practices

---

## Database

Planned:

- PostgreSQL

---

## ORM

Planned:

- SQLx

---

## CI/CD

=======
### Sprint 1
- Config ✅
- Shared ✅
- Errors ✅
- Observability ✅

### Sprint 2 - Auth Service

Implemented:

- User Creation ✅
- Email Validation ✅
- Password Validation ✅
- Argon2id Hashing ✅
- Password Verification ✅
- Login Flow ✅
- JWT Access Token ✅
- JWT Validation ✅
- Refresh Token ✅
- Refresh Token Validation ✅
- Duplicate User Protection ✅

## Technology

- Rust
- Tokio
- JWT
- Argon2id
>>>>>>> 77d1543 (docs(readme): automatic update)
- GitHub Actions
- PostgreSQL (planned)
- SQLx (planned)
- Docker (planned)
- Kubernetes (planned)

<<<<<<< HEAD
---

## Containerization

Planned:
=======
## Future Modules
>>>>>>> 77d1543 (docs(readme): automatic update)

- IAM
- Wallet
- KYC
- Ledger
- Payment Gateway
- Tokenization
- Risk Engine

<<<<<<< HEAD
---

## Orchestration

Planned:
=======
## Repository
>>>>>>> 77d1543 (docs(readme): automatic update)

Private Repository

BlockX is under active development.

<<<<<<< HEAD
# Roadmap

## Auth Service

- User Entity ✅
- Create User ✅
- Password Hashing ✅
- Login Flow ✅
- JWT Generation ✅
- JWT Validation ✅
- Duplicate User Protection ✅
- Refresh Tokens 🔄
- RBAC 🔄
- Session Management 🔄
- PostgreSQL Persistence 🔄

---

## IAM

- Roles
- Permissions
- Multi Tenant Support
- Organization Management

---

## Wallet

- Wallet Service
- Asset Storage
- Transaction History

---

## KYC

- Identity Verification
- Compliance Workflow
- Risk Classification

---

## Ledger

- Double Entry Accounting
- Audit Trail
- Financial Reconciliation

---

## Payment Gateway

- Payment Processing
- Merchant Accounts
- Settlement Engine
- Transaction Monitoring

---

## Tokenization

- Digital Assets
- RWA Support
- Compliance Controls

---

# Quality Assurance

Current validations:

```bash
cargo check
cargo test
```

GitHub Actions runs validation automatically on every push.

---

# Contributors

## Marco Antonio Conceicao

Co-Founder

Responsible for:

- Product Vision
- Platform Architecture
- Infrastructure Strategy

---

## Saulo

Co-Founder

Responsible for:

- Product Development
- Engineering Collaboration
- Platform Evolution

---

# Repository

Private Repository

BlockX is currently under active development.

---

# License

Private Project

All Rights Reserved.

---

## BlockX

**Building the infrastructure layer for secure digital finance.**
=======
>>>>>>> 77d1543 (docs(readme): automatic update)
