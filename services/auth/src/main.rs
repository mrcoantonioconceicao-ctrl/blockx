use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod api;
mod application;
mod domain;
mod infrastructure;
mod bootstrap;
mod state;

use state::AppState;
use crate::application::{create_user, login_user};
use crate::application::refresh_flow;

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    user_id: String,
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {

    let user = match create_user::execute(payload.email, payload.password) {
        Ok(u) => u,
        Err(_) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json("failed to create user"),
            );
        }
    };

    state.repository.save(&user);

    (
        axum::http::StatusCode::CREATED,
        Json(RegisterResponse {
            user_id: user.id,
        }),
    )
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {

    let user = match state.repository.find_by_email(&payload.email) {
        Some(u) => u,
        None => {
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                Json("invalid credentials"),
            );
        }
    };

    match login_user::execute(&user) {
        Ok(tokens) => {
            // salva refresh token no store (IMPORTANTE)
            state.refresh_store.save(crate::application::refresh_token_model::RefreshToken {
                token: tokens.refresh_token.clone(),
                user_id: user.id.clone(),
                exp: 0, // já controlado no service
            });

            (
                axum::http::StatusCode::OK,
                Json(tokens),
            )
        }
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json("login failed"),
        ),
    }
}

async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> impl IntoResponse {

    match refresh_flow::execute(
        &state.refresh_store,
        &payload.refresh_token,
    ) {
        Ok(tokens) => (
            axum::http::StatusCode::OK,
            Json(tokens),
        ),
        Err(e) => (
            axum::http::StatusCode::UNAUTHORIZED,
            Json(e),
        ),
    }
}

#[tokio::main]
async fn main() {

    let state = AppState::new();

    let app = Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Auth running on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();
}
