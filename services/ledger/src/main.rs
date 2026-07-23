use axum::{
    Json, Router, debug_handler,
    extract::{Path, State},
    routing::{get, post},
};
use serde::Serialize;
use std::net::SocketAddr;
use uuid::Uuid;

use ledger::{application, domain, infrastructure};

use application::{
    chart_of_accounts_service::ChartOfAccountsService, journal_service::JournalService,
    ledger_service::LedgerService,
};

use domain::{Account, Journal};

use infrastructure::{
    in_memory_journal_repository::InMemoryJournalRepository,
    in_memory_ledger_repository::InMemoryLedgerRepository,
};

#[derive(Clone)]
struct AppState {
    ledger_service: LedgerService<InMemoryLedgerRepository>,
    journal_service: JournalService<InMemoryJournalRepository>,
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

async fn list_accounts(State(state): State<AppState>) -> Json<Vec<Account>> {
    Json(state.chart_service.all().into_iter().cloned().collect())
}

async fn create_journal(
    State(state): State<AppState>,
    Json(journal): Json<Journal>,
) -> Result<Json<Journal>, String> {
    state.journal_service.create(journal.clone())?;
    Ok(Json(journal))
}

#[debug_handler]
async fn list_journals(State(state): State<AppState>) -> Json<Vec<Journal>> {
    Json(state.journal_service.list())
}

#[debug_handler]
async fn get_journal(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Journal>, String> {
    let uuid = Uuid::parse_str(&id).map_err(|_| "UUID inválido.".to_string())?;

    state
        .journal_service
        .find(uuid)
        .map(Json)
        .ok_or_else(|| "Journal não encontrado.".to_string())
}

#[tokio::main]
async fn main() {
    let ledger_repository = InMemoryLedgerRepository::new();
    let journal_repository = InMemoryJournalRepository::new();

    let ledger_service = LedgerService::new(ledger_repository);
    let journal_service = JournalService::new(journal_repository);

    let chart_service = ChartOfAccountsService::new();

    let state = AppState {
        ledger_service,
        journal_service,
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
