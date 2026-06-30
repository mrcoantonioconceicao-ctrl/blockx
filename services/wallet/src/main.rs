use axum::{
    routing::{get, post},
    Json, Router,
    extract::{State, Path},
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};

mod domain;
mod infrastructure;
mod application;

use application::wallet_service::WalletService;
use infrastructure::wallet_repository::InMemoryWalletRepository;

#[derive(Clone)]
struct AppState {
    service: Arc<WalletService<InMemoryWalletRepository>>,
}

impl AppState {
    fn new() -> Self {
        let repo = InMemoryWalletRepository::new();
        let service = WalletService::new(repo);

        Self {
            service: Arc::new(service),
        }
    }
}

#[derive(Deserialize)]
struct CreateWalletRequest {
    user_id: String,
    currency: String,
}

#[derive(Deserialize)]
struct AmountRequest {
    user_id: String,
    amount: i64,
}

#[derive(Serialize)]
struct WalletResponse {
    id: String,
    user_id: String,
    currency: String,
    balance: i64,
    status: String,
}

async fn create_wallet(
    State(state): State<AppState>,
    Json(req): Json<CreateWalletRequest>,
) -> Json<WalletResponse> {

    let wallet = state.service.create_wallet(req.user_id, req.currency);

    Json(WalletResponse {
        id: wallet.id,
        user_id: wallet.user_id,
        currency: wallet.currency,
        balance: wallet.balance,
        status: format!("{:?}", wallet.status),
    })
}

async fn credit(
    State(state): State<AppState>,
    Json(req): Json<AmountRequest>,
) -> Json<WalletResponse> {

    let wallet = state.service.credit(&req.user_id, req.amount).unwrap();

    Json(WalletResponse {
        id: wallet.id,
        user_id: wallet.user_id,
        currency: wallet.currency,
        balance: wallet.balance,
        status: format!("{:?}", wallet.status),
    })
}

async fn debit(
    State(state): State<AppState>,
    Json(req): Json<AmountRequest>,
) -> Json<WalletResponse> {

    let wallet = state.service.debit(&req.user_id, req.amount).unwrap();

    Json(WalletResponse {
        id: wallet.id,
        user_id: wallet.user_id,
        currency: wallet.currency,
        balance: wallet.balance,
        status: format!("{:?}", wallet.status),
    })
}

async fn get_wallet(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Json<Option<WalletResponse>> {

    let wallet = state.service.get_wallet(&user_id);

    Json(wallet.map(|w| WalletResponse {
        id: w.id,
        user_id: w.user_id,
        currency: w.currency,
        balance: w.balance,
        status: format!("{:?}", w.status),
    }))
}

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .route("/wallet/create", post(create_wallet))
        .route("/wallet/credit", post(credit))
        .route("/wallet/debit", post(debit))
        .route("/wallet/:user_id", get(get_wallet))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 4001));

    println!("Wallet running on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
