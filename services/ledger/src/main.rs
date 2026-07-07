use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod application;
mod domain;
mod infrastructure;

use application::ledger_service::LedgerService;
use domain::{Journal, LedgerEntry};
use infrastructure::in_memory_ledger_repository::InMemoryLedgerRepository;

#[derive(Clone)]
struct AppState {
    service: LedgerService<InMemoryLedgerRepository>,
}

#[derive(Deserialize)]
struct CreateEntryRequest {
    debit_account: String,
    credit_account: String,
    amount: String,
    currency: String,
    description: String,
}

#[derive(Serialize)]
struct HealthResponse {
    service: String,
    status: String,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "Nexavor Ledger".to_string(),
        status: "running".to_string(),
    })
}

async fn create_entry(
    State(_state): State<AppState>,
    Json(_request): Json<CreateEntryRequest>,
) -> Result<Json<String>, String> {
    Err(
        "Endpoint em refatoração para o novo modelo Journal."
            .to_string(),
    )
}

async fn list_entries(
    State(state): State<AppState>,
) -> Json<Vec<LedgerEntry>> {
    Json(state.service.list_entries())
}

#[tokio::main]
async fn main() {
    let repository = InMemoryLedgerRepository::new();

    let service = LedgerService::new(repository);

    let state = AppState { service };

    let app = Router::new()
        .route("/", get(health))
        .route("/entries", post(create_entry).get(list_entries))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4002));

    println!("Nexavor Ledger running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
