# BlockX

> Enterprise Financial Platform written in Rust, built with SOA, Clean Architecture and Domain-Driven Design.

---

# Vision

BlockX is becoming a modular financial platform capable of powering:

- Digital Wallets
- Banking as a Service (BaaS)
- Embedded Finance
- Crypto Payment Gateway
- Tokenization
- Risk Management
- Compliance
- Audit
- Ledger
- Future Open Finance integrations

The project is designed as a long-term enterprise platform rather than a simple banking application.

---

# Current Architecture

```
                BlockX Platform

                      API Gateway
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
     Auth Service      Wallet Service    Ledger Service
        │                  │                  │
        │                  │                  │
        └────────────── HTTP ────────────────┘
                           │
                    Future Database
              PostgreSQL / CockroachDB
```

---

# Workspace

```
blockx/

crates/
├── shared
├── config
├── observability
└── errors

services/
├── auth
├── wallet
├── ledger
├── gateway (planned)
├── risk (planned)
├── audit (planned)
├── iam (planned)
├── kyc (planned)
└── tokenization (planned)
```

---

# Services

## Auth

Responsible for:

- Registration
- Login
- Password Hashing
- JWT
- Refresh Token

---

## Wallet

Responsible for:

- Wallet creation
- Wallet management
- Currency ownership
- Wallet state

Current status:

Wallet still maintains balances locally.

Future state:

Wallet will become a consumer of the Ledger Service, no longer updating balances directly.

---

## Ledger

The Ledger is becoming the financial core of BlockX.

Responsibilities:

- Double-entry accounting
- Journals
- Debit/Credit entries
- Atomic transactions
- Immutable financial history

Future responsibilities:

- Chart of Accounts
- Posting engine
- Accounting validation
- Settlement support

---

# Architectural Evolution

Current

```
Wallet
    │
updates balance
```

Future

```
Wallet
    │
HTTP
    ▼
Ledger
    │
creates Journal
    │
validates Double Entry
    │
persists accounting entries
    │
returns success
    │
Wallet refreshes balance
```

---

# Next Milestones

- Ledger REST API
- Journal endpoints
- Chart of Accounts
- Atomic Transactions
- Wallet → Ledger integration
- PostgreSQL persistence
- Crypto Gateway integration
- Event Bus
- Audit Service
- Risk Engine

---

# Principles

- Rust
- SOA
- Clean Architecture
- DDD
- CQRS (future)
- Event Driven Architecture (future)
- Crypto Agility
- Post-Quantum Ready

---

# Long-Term Goal

BlockX aims to become a production-grade financial platform capable of supporting digital banks, fintechs, payment gateways, embedded finance solutions and crypto infrastructure while remaining modular, secure and highly scalable.9
