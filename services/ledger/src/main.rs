use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use serde::Serialize;
use std::net::SocketAddr;

use ledger::{
    application::{
        chart_of_accounts_service::ChartOfAccountsService, journal_service::JournalService,
        ledger_service::LedgerService,
    },
    domain::{Account, Journal},
    infrastructure::{
        in_memory_journal_repository::InMemoryJournalRepository,
        in_memory_ledger_repository::InMemoryLedgerRepository,
    },
};

#[derive(Clone)]
struct AppState {
    ledger_service: LedgerService<InMemoryLedgerRepository>,
    journal_service: JournalService,
    chart_service: ChartOfAccountsService,
}

#[derive(Serialize)]
struct HealthResponse {
    service: String,
    status: String,
}

#[derive(Serialize)]
struct AuditResponse {
    journals: usize,
    balanced: bool,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "Nexavor Ledger".into(),
        status: "running".into(),
    })
}

async fn list_accounts(State(state): State<AppState>) -> Json<Vec<Account>> {
    Json(state.chart_service.all().into_iter().cloned().collect())
}

async fn create_journal(State(_state): State<AppState>) -> Result<Json<String>, String> {
    Err("Journal API em desenvolvimento.".into())
}

async fn list_journals(State(state): State<AppState>) -> Json<Vec<Journal>> {
    Json(state.journal_service.list())
}

async fn get_journal(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Journal>, String> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|_| "UUID inválido".to_string())?;

    match state.journal_service.find(uuid) {
        Some(journal) => Ok(Json(journal)),
        None => Err("Journal não encontrado.".into()),
    }
}

async fn audit_ledger(State(state): State<AppState>) -> Json<AuditResponse> {
    let journals = state.journal_service.list();

    let balanced = journals.iter().all(|j| j.validate().is_ok());

    Json(AuditResponse {
        journals: journals.len(),
        balanced,
    })
}

#[tokio::main]
async fn main() {
    let ledger_repository = InMemoryLedgerRepository::new();
    let journal_repository = InMemoryJournalRepository::new();

    let state = AppState {
        ledger_service: LedgerService::new(ledger_repository),
        journal_service: JournalService::new(journal_repository),
        chart_service: ChartOfAccountsService::new(),
    };

    let app = Router::new()
        .route("/", get(health))
        .route("/accounts", get(list_accounts))
        .route("/journals", get(list_journals))
        .route("/journals", post(create_journal))
        .route("/journals/:id", get(get_journal))
        .route("/ledger/audit", get(audit_ledger))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4002));

    println!("Nexavor Ledger running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
