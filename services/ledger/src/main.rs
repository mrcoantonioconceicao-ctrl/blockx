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
use infrastructure::in_memory_ledger_repository::InMemoryLedgerRepository;

#[derive(Clone)]
struct AppState {
    service: LedgerService<InMemoryLedgerRepository>,
}

#[derive(Deserialize)]
struct CreateEntryRequest {
    debit_account: String,
    credit_account: String,
    amount: f64,
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
        service: "BlockX Ledger".to_string(),
        status: "running".to_string(),
    })
}

async fn create_entry(
    State(state): State<AppState>,
    Json(request): Json<CreateEntryRequest>,
) -> Json<domain::ledger_entry::LedgerEntry> {
    let entry = state.service.record_entry(
        request.debit_account,
        request.credit_account,
        request.amount,
        request.currency,
        request.description,
    );

    Json(entry)
}

async fn list_entries(
    State(state): State<AppState>,
) -> Json<Vec<domain::ledger_entry::LedgerEntry>> {
    Json(state.service.all_entries())
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

    println!("Ledger running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
