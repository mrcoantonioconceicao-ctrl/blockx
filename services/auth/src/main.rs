use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
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
use crate::infrastructure::user_repository::UserRepository;

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

#[derive(Serialize)]
struct RegisterResponse {
    user_id: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Json<RegisterResponse> {

    let user = create_user::execute(
        payload.email,
        payload.password,
    ).expect("create user failed");

    UserRepository::save(&*state.repository, &user);

    Json(RegisterResponse {
        user_id: user.id.to_string(),
    })
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Json<LoginResponse> {

    let user = state.repository
        .find_by_email(&payload.email)
        .expect("user not found");

    // aqui usamos seu login_user (JWT)
    let token = login_user::execute(&user)
        .expect("login failed");

    Json(LoginResponse { token })
}

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Auth running on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
