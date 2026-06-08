
# BlockX

Enterprise Financial Infrastructure Platform built in Rust.

BlockX is a next-generation financial security platform designed with an enterprise-first architecture focused on auditability, security, compliance, scalability, and digital asset infrastructure.

---

# Vision

BlockX aims to provide the foundational infrastructure required for modern financial systems, digital asset platforms, tokenization services, identity management, compliance operations, and institutional-grade payment solutions.

The platform is being built from the ground up with a deterministic core architecture, emphasizing:

- Security by Design
- Auditability
- Scalability
- Compliance Readiness
- Modular Services
- Cloud Native Deployment
- High Performance Rust Services

---

# Current Development Status

### Phase

Foundation Development

### Current Sprint

Auth Service Evolution

### Progress

#### Sprint 0 — Foundation ✅

- Monorepo structure
- Workspace configuration
- Core crates
- Environment support
- Shared utilities
- Error handling
- Observability foundation

#### Sprint 1 — Platform Foundations ✅

- Config crate
- Shared crate
- Errors crate
- Observability crate
- GitHub Actions CI

#### Sprint 2 — Auth Service (In Progress) 🚀

Implemented:

- User domain model
- User creation flow
- Password hashing
- Password verification
- Login flow
- Repository abstraction
- In-memory repository
- Repository lookup by email
- Unit tests

---

# Current Auth Capabilities

## User Creation

Users can be created through the application layer.

Features:

- Unique UUID generation
- Email support
- Password hashing
- Domain entity creation

---

## Password Security

BlockX uses:

- Argon2id

Password storage is performed using cryptographic hashing and random salts.

Example:

```text
$argon2id$v=19$m=19456,t=2,p=1$...
```

Passwords are never stored in plaintext.

---

## Login Flow

Current login process:

1. User lookup by email
2. Password verification
3. Authentication result

Implemented in:

```text
services/auth/src/application/login_user.rs
```

---

## Repository Pattern

Current repository abstraction:

```rust
pub trait UserRepository {
    fn save(&self, user: &User);

    fn find_by_email(
        &self,
        email: &str,
    ) -> Option<User>;
}
```

Current implementation:

```text
InMemoryUserRepository
```

Future implementation:

```text
PostgreSQLUserRepository
```

---

# Architecture

```text
BlockX
│
├── apps
│
├── services
│   └── auth
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

- Rust

## Backend

- Axum
- Tokio

## Security

- Argon2id

## Database (Planned)

- PostgreSQL

## ORM

- SQLx

## CI/CD

- GitHub Actions

## Containerization (Planned)

- Docker

## Orchestration (Planned)

- Kubernetes

---

# Design Principles

BlockX follows enterprise engineering principles:

### Security First

Security is treated as a core feature.

### Auditability

All critical operations must be traceable.

### Deterministic Core

Business rules must remain predictable and verifiable.

### Modular Architecture

Services evolve independently.

### Infrastructure as Code

Deployments should be automated and reproducible.

---

# Development Roadmap

## Auth Service

- [x] User Entity
- [x] Create User
- [x] Password Hashing
- [x] Password Verification
- [x] Login Flow
- [x] Repository Lookup
- [ ] JWT Authentication
- [ ] Access Tokens
- [ ] Refresh Tokens
- [ ] RBAC
- [ ] Session Management
- [ ] PostgreSQL Persistence

---

## IAM

- [ ] Roles
- [ ] Permissions
- [ ] Multi-Tenant Support
- [ ] Organization Management

---

## Wallet

- [ ] Wallet Service
- [ ] Asset Storage
- [ ] Transaction History

---

## KYC

- [ ] Identity Verification
- [ ] Compliance Workflow
- [ ] Risk Classification

---

## Ledger

- [ ] Double Entry Accounting
- [ ] Audit Trail
- [ ] Financial Reconciliation

---

## Tokenization

- [ ] Digital Assets
- [ ] RWA Support
- [ ] Compliance Controls

---

# Quality Assurance

Current automated validation:

```bash
cargo check
cargo test
```

GitHub Actions executes validation automatically on every push.

---

# Contributors

## Marco Antônio Conceição

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

GitHub:

https://github.com/mrcoantonioconceicao-ctrl/blockx

---

# License

Private Project

All rights reserved.

---

# BlockX

Building the infrastructure layer for secure digital finance.