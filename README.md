BlockX - Auth Service (Rust)
BlockX Auth Service é um microserviço de autenticação de alta performance construído em Rust utilizando Axum, Clean Architecture e JWT.
ARQUITETURA
O sistema segue Clean Architecture com separação clara de camadas:
API (Axum) ↓ Application Layer (casos de uso) ↓ Domain Layer (regras de negócio) ↓ Infrastructure Layer (repositórios)
FUNCIONALIDADES
Registro de usuários com hash seguro de senha (Argon2)
Login com validação de senha
Geração de JWT
Repositório em memória (extensível para PostgreSQL)
Estrutura baseada em Clean Architecture
API HTTP assíncrona com Axum
Separação de responsabilidades (SOA)
STACK TECNOLÓGICA
Rust (edição 2024)
Axum (framework HTTP)
Tokio (runtime assíncrono)
Argon2 (hash de senha)
JSON Web Token (jsonwebtoken)
Serde (serialização JSON)
ENDPOINTS DA API
REGISTRO DE USUÁRIO
POST /auth/register
Exemplo de request:
{ "email": "user@blockx.io", "password": "123456" }
Resposta:
{ "user_id": "uuid" }
LOGIN
POST /auth/login
Exemplo de request:
{ "email": "user@blockx.io", "password": "123456" }
Resposta:
{ "token": "jwt_token" }
COMO EXECUTAR
Executar o serviço:
cargo run -p auth
O servidor será iniciado em:
http://0.0.0.0:3000⁠�
FILOSOFIA DE ARQUITETURA
Nenhuma regra de negócio dentro da camada HTTP
Separação estrita de responsabilidades
Código orientado a domínio
Sistema stateless (JWT)
Repositórios substituíveis (in-memory ou banco real)
MÓDULOS FUTUROS
Wallet Service
Ledger (contabilidade dupla)
KYC Engine
Risk Engine
Payment Gateway
Tokenização de ativos
STATUS DO PROJETO
Projeto em desenvolvimento ativo. Base sólida pronta para evolução para sistema financeiro completo.
