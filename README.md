# Nexavor

Enterprise Financial Platform written in Rust.

## Current Architecture

```
services/
├── auth
├── ledger
└── wallet
```

## Ledger

Current features:

- In-memory Ledger Repository
- Double Entry domain
- Journal entity
- Chart of Accounts
- Default Chart of Accounts
- Account Types
- REST API

Endpoints

GET /

Health Check

GET /accounts

Returns the Chart of Accounts.

GET /entries

Returns ledger entries.

POST /entries

Reserved for Journal posting (under refactoring).

## Current Chart of Accounts

| Code | Account |
|------|----------|
|1001|Cash|
|1100|Bank|
|2000|Accounts Payable|
|3000|Equity|
|4000|Revenue|
|5000|Expense|

## Status

✔ Auth Service

✔ Wallet Service

✔ Ledger Service

✔ Chart of Accounts

✔ REST API

🚧 Journal Posting

🚧 Persistence

🚧 PostgreSQL

🚧 Audit Trail

## Build

```bash
cargo build
```

Run Ledger

```bash
cargo run -p ledger
```

API

```
http://localhost:4002
```

Project under active development.
