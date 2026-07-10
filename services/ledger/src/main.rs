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

use application::{
    chart_of_accounts_service::ChartOfAccountsService,
    ledger_service::LedgerService,
};
use domain::{Account, LedgerEntry};
use infrastructure::in_memory_ledger_repository::InMemoryLedgerRepository;

#[derive(Clone)]
struct AppState {
    ledger_service: LedgerService<InMemoryLedgerRepository>,
    chart_service: ChartOfAccountsService,
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

async fn list_accounts(
    State(state): State<AppState>,
) -> Json<Vec<Account>> {
    Json(
        state
            .chart_service
            .all()
            .into_iter()
            .cloned()
            .collect(),
    )
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
    Json(state.ledger_service.list_entries())
}

#[tokio::main]
async fn main() {
    let repository = InMemoryLedgerRepository::new();

    let ledger_service = LedgerService::new(repository);

    let chart_service = ChartOfAccountsService::new();

    let state = AppState {
        ledger_service,
        chart_service,
    };

    let app = Router::new()
        .route("/", get(health))
        .route("/accounts", get(list_accounts))
        .route("/entries", post(create_entry).get(list_entries))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4002));

    println!("Nexavor Ledger running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
