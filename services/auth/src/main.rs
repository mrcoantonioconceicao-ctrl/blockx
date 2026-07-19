use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod api;
mod application;
mod bootstrap;
mod domain;
mod infrastructure;
mod state;

use crate::application::{
    create_user,
    login_user,
    refresh_flow,
};
use crate::infrastructure::user_repository::UserRepository;
use state::AppState;

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

#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Serialize)]
struct RefreshResponse {
    access_token: String,
    refresh_token: String,
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, String)> {

    if state.repository.exists(&payload.email) {
        return Err((
            StatusCode::CONFLICT,
            "email already registered".to_string(),
        ));
    }

    let user = create_user::execute(
        payload.email,
        payload.password,
    )
    .map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "could not create user".to_string(),
        )
    })?;

    state.repository.save(&user);

    Ok(Json(RegisterResponse {
        user_id: user.id.to_string(),
    }))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {

    let user = state
        .repository
        .find_by_email(&payload.email)
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "invalid credentials".to_string(),
        ))?;

    let result = login_user::execute(
        &user,
        &payload.password,
    )
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "invalid credentials".to_string(),
        )
    })?;

    state
        .refresh_store
        .save(result.refresh_token.clone());

    Ok(Json(LoginResponse {
        access_token: result.access_token,
        refresh_token: result.refresh_token.token,
    }))
}

async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, (StatusCode, String)> {

    let result = refresh_flow::execute(
        &state.refresh_store,
        &payload.refresh_token,
    )
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "invalid refresh token".to_string(),
        )
    })?;

    Ok(Json(RefreshResponse {
        access_token: result.access_token,
        refresh_token: result.refresh_token,
    }))
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
