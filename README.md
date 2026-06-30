# 🚀 BlockX

Enterprise-grade modular financial system built in Rust.

---

# 🧠 Architecture Overview

BlockX is designed using modern backend principles:

- Domain Driven Design (DDD)
- Clean Architecture
- Service Oriented Architecture (SOA)
- Event-driven financial modeling (future-ready)

The goal is to evolve into a **distributed financial core system** with strong guarantees of consistency and auditability.

---

# 📦 Services

## 🔐 Auth Service

Responsible for identity and authentication:

- User registration
- Login flow
- Password hashing (Argon2)
- JWT generation and validation

---

## 💰 Wallet Service (Active)

A core financial wallet system.

### Features

- Wallet creation per user
- Credit (deposit funds)
- Debit (withdraw funds with validation)
- Balance tracking
- Status lifecycle:
  - Active
  - Frozen
  - Closed

### Architecture

- Domain Layer → Wallet entity and rules
- Application Layer → WalletService (business logic)
- Infrastructure Layer → In-memory repository
- API Layer → Axum HTTP server

### Running

http://localhost:4001

---

# 🧱 Workspace Structure
blockx/ ├── services/ │   ├── auth/ │   └── wallet/ ├── crates/ │   ├── config/ │   ├── errors/ │   ├── observability/ │   └── shared/ ├── infrastructure/ ├── docs/ ├── scripts/

---

# 🧩 Core Principles

- Explicit domain modeling
- Separation of concerns
- Stateless application services
- Repository abstraction for persistence
- Memory-first implementation (ready for Postgres upgrade)

---

# 🚀 Roadmap

## Phase 1 — Core Services (current)
- Auth Service
- Wallet Service

## Phase 2 — Financial Core
- Ledger (double-entry accounting system)
- Transaction immutability layer
- Audit log engine

## Phase 3 — Platform Layer
- Risk Engine
- Event Bus
- ORION orchestration layer

## Phase 4 — Distributed Systems
- PostgreSQL persistence
- Message queues
- Distributed consistency

---

# 🧪 Status

| Service | Status |
|--------|--------|
| Auth   | ✅ Active |
| Wallet | ✅ Active |
| Ledger | ⏳ Next |
| ORION  | ⏳ Planned |

---

# 👤 Author

Marco Antonio Conceicao
