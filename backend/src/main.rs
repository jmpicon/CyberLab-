mod orchestrator;
mod pty;
mod state;

use crate::state::PlayerState;
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
    routing::{get, post},
    Router, Json,
};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use tracing_subscriber;
use serde_json::json;

struct AppState {
    player: Mutex<PlayerState>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let shared_state = Arc::new(AppState {
        player: Mutex::new(PlayerState::load("savegame.json").unwrap_or_else(|_| PlayerState::new())),
    });

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/player", get(get_player_state))
        .route("/api/mission/start", post(start_mission))
        .route("/ws/terminal", get(terminal_handler))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

async fn get_player_state(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let player = state.player.lock().unwrap();
    Json(json!( *player ))
}

async fn start_mission() -> impl IntoResponse {
    // Logic to call orchestrator.create_mission_container
    Json(json!({ "status": "started", "container_id": "mock-id" }))
}

async fn terminal_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // This will connect WebSocket to Docker Exec stream or local PTY
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    tracing::info!("received: {}", t);
                    let _ = socket.send(Message::Text(format!("Echo: {}", t))).await;
                }
                _ => {}
            }
        } else {
            break;
        }
    }
}
