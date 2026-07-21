use axum::{
    debug_handler,
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;

use ledger::{
    application,
    domain,
    infrastructure,
};

use application::{
    chart_of_accounts_service::ChartOfAccountsService,
    ledger_service::LedgerService,
};
use domain::{Account, Journal};
use infrastructure::in_memory_ledger_repository::InMemoryLedgerRepository;

#[derive(Clone)]
struct AppState {
    ledger_service: LedgerService<InMemoryLedgerRepository>,
    chart_service: ChartOfAccountsService,
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
    Json(state.chart_service.all().into_iter().cloned().collect())
}

async fn create_journal(
    State(_state): State<AppState>,
) -> Result<Json<String>, String> {
    Err("Journal API em desenvolvimento.".to_string())
}

#[debug_handler]
async fn list_journals(
    State(_state): State<AppState>,
) -> Json<Vec<Journal>> {
    Json(vec![])
}

#[debug_handler]
async fn get_journal(
    Path(_id): Path<String>,
    State(_state): State<AppState>,
) -> Result<Json<Journal>, String> {
    Err("Journal não encontrado.".to_string())
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
        .route("/journals", post(create_journal))
        .route("/journals", get(list_journals))
        .route("/journals/:id", get(get_journal))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4002));

    println!("Nexavor Ledger running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
