# BlockX

> Plataforma financeira modular construída em Rust com foco em alta performance, segurança, arquitetura empresarial e execução determinística.

---

# Visão Geral

O BlockX é uma plataforma financeira baseada em microsserviços seguindo os princípios de:

- Clean Architecture
- Domain-Driven Design (DDD)
- SOA / Microservices
- Event Driven (próximas versões)
- Segurança por padrão
- Crypto Agility
- Preparação para criptografia pós-quântica

---

# Arquitetura Atual

```
                    BlockX

        +-------------------------+
        |        API Gateway      |
        +-----------+-------------+
                    |
    +---------------+----------------+
    |               |                |
    ▼               ▼                ▼

 Auth Service   Wallet Service   Ledger Service
     │               │                │
 JWT / IAM      Carteiras       Livro Contábil
 Refresh        Saldos           Auditoria

```

---

# Serviços Implementados

## Auth Service

Status:

- ✅ Registro de usuários
- ✅ Login
- ✅ JWT
- ✅ Refresh Token
- ✅ Repositório em memória
- ✅ Clean Architecture

---

## Wallet Service

Status:

- ✅ Criação de Wallet
- ✅ Estrutura de domínio
- ✅ Repositório em memória
- ✅ Serviço de aplicação
- ✅ API HTTP

---

## Ledger Service

Status:

- ✅ LedgerEntry
- ✅ LedgerRepository
- ✅ InMemoryLedgerRepository
- ✅ LedgerService
- ✅ API HTTP
- ✅ Registro de lançamentos
- ✅ Consulta de lançamentos

---

# Estrutura do Projeto

```
blockx/

crates/
config/
errors/
observability/
shared/

services/

auth/
wallet/
ledger/

docs/
scripts/
deployments/
infrastructure/
```

---

# Roadmap

## Fase Atual

- ✅ Auth
- ✅ Wallet
- ✅ Ledger

---

## Próximos Serviços

- KYC
- IAM
- Risk Engine
- Audit
- Notification
- Tokenization
- Gateway Cripto
- Payment Gateway
- PIX
- Open Finance

---

# Objetivos

O BlockX está sendo desenvolvido para servir como base para:

- Banco Digital
- Banking as a Service (BaaS)
- Embedded Finance
- Gateway de Pagamentos
- Gateway Cripto
- Carteiras Digitais
- Stablecoins
- Tokenização de Ativos
- Open Finance

---

# Tecnologias

- Rust
- Axum
- Tokio
- UUID
- Serde
- Chrono

---

# Arquitetura

- Clean Architecture
- DDD
- SOLID
- Repository Pattern
- Service Layer
- Dependency Inversion

---

# Próximos Marcos

- Double Entry Accounting
- PostgreSQL
- CockroachDB
- Event Bus
- Kafka/NATS
- gRPC
- Observabilidade
- Kubernetes
- Alta Disponibilidade

---

# Licença

MIT

---

Desenvolvido como parte do ecossistema **BlockX**.
