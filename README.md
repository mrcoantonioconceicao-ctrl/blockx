# BlockX

Enterprise-Grade Financial Infrastructure Platform

BlockX is a next-generation financial infrastructure platform built with Rust, designed to deliver security, auditability, compliance, and scalability for modern financial applications.

The platform follows an enterprise-first architecture, combining deterministic services with intelligent automation to support digital payments, wallet infrastructure, compliance operations, asset tokenization, and financial ecosystems.

---

# Vision

Our mission is to build a secure and scalable financial infrastructure capable of supporting:

- Authentication & Identity Management
- Wallet Infrastructure
- KYC & Compliance
- Ledger & Audit Systems
- Risk Management
- Financial Operations
- Digital Asset Infrastructure
- Asset Tokenization
- Enterprise Integrations

BlockX is designed with security, transparency, and long-term sustainability as core principles.

---

# Architecture

The platform follows a modular monorepo architecture.

```text
blockx/
├── apps/
├── crates/
│   ├── config
│   ├── errors
│   ├── observability
│   └── shared
│
├── services/
│   └── auth
│
├── deployments/
├── docs/
└── infrastructure/
```

Each service is developed independently while sharing common platform foundations.

---

# Current Development Status

## Sprint 0 — Foundation

Completed:

- ✅ Cargo Workspace
- ✅ Configuration Foundation
- ✅ Error Handling Foundation
- ✅ Observability Foundation
- ✅ Shared Utilities Foundation

---

## Sprint 1 — Auth Service Foundation

Completed:

- ✅ User Domain Model
- ✅ Repository Pattern
- ✅ In-Memory Repository
- ✅ UUID Generation
- ✅ Bootstrap Layer
- ✅ Service Startup Flow

---

## Sprint 2 — Authentication Security

Completed:

- ✅ Password Service
- ✅ Argon2id Password Hashing
- ✅ Secure Password Storage
- ✅ User Creation Flow

In Progress:

- 🔄 Password Verification
- 🔄 Login Use Case

---

# Roadmap

## Authentication & IAM

- Password Verification
- Login Flow
- JWT Authentication
- Refresh Tokens
- Session Management
- Role-Based Access Control (RBAC)
- Identity and Access Management (IAM)

## Persistence Layer

- PostgreSQL Integration
- SQLx Repositories
- Database Migrations
- Repository Implementations

## Financial Core

- Wallet Service
- Transaction Engine
- Audit Service
- Ledger Infrastructure
- Risk Engine

## Compliance Layer

- KYC Service
- AML Monitoring
- Compliance Workflows
- Regulatory Reporting

## Digital Assets

- Tokenization Service
- Digital Asset Infrastructure
- Asset Registry
- Settlement Services

---

# Technology Stack

Current:

- Rust
- Cargo Workspace
- Argon2id
- UUID
- GitHub Actions

Planned:

- Axum
- Tokio
- PostgreSQL
- SQLx
- OpenTelemetry
- Docker
- Kubernetes

---

# Engineering Principles

BlockX is built around the following principles:

- Security First
- Auditability by Design
- Enterprise Scalability
- Deterministic Core Services
- Modular Architecture
- Compliance Ready
- Cloud Native Infrastructure
- Long-Term Maintainability

---

# CI/CD

Continuous Integration is executed through GitHub Actions.

Current pipeline:

- Cargo Check
- Cargo Test

Future pipeline:

- Clippy
- Rustfmt
- Security Scanning
- Dependency Auditing
- Automated Releases

---

# Founders

- Marco Antônio Conceição
- Saulo

---

# Ownership

BlockX is a proprietary project developed by its founders.

All rights reserved.
