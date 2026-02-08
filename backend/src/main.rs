mod orchestrator;
mod pty;
mod state;

use futures::{SinkExt, StreamExt};
use crate::state::PlayerState;
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::{IntoResponse, Html, Redirect},
    routing::{get, post},
    Router, Json,
};
use tower_http::cors::CorsLayer;
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
        .route("/", get(root_handler))
        .route("/api/health", get(health_check))
        .route("/api/missions", get(list_missions))
        .route("/api/player", get(get_player_state))
        .route("/api/mission/start", post(start_mission))
        .route("/ws/terminal", get(terminal_handler))
        .layer(CorsLayer::permissive())
        .with_state(shared_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> impl IntoResponse {
    // Automatically redirect browser users to the graphical UI port
    Redirect::temporary("http://localhost:5173")
}

async fn health_check() -> impl IntoResponse {
    Json(json!({ "status": "ok", "service": "CyberLab Orchestrator" }))
}

async fn list_missions() -> impl IntoResponse {
    let mut missions = Vec::new();
    let paths = std::fs::read_dir("missions/base_pack").unwrap();
    for path in paths {
        if let Ok(entry) = path {
            if entry.path().extension().map_or(false, |ext| ext == "yaml") {
                let content = std::fs::read_to_string(entry.path()).unwrap();
                let mission: serde_json::Value = serde_yaml::from_str(&content).unwrap();
                missions.push(mission);
            }
        }
    }
    Json(missions)
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

use crate::pty::PtySession;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::os::unix::io::FromRawFd;

async fn handle_socket(mut socket: WebSocket) {
    let pty = match PtySession::new("/bin/bash", &["--login"]) {
        Ok(p) => p,
        Err(e) => {
            let _ = socket.send(Message::Text(format!("Failed to spawn PTY: {}", e))).await;
            return;
        }
    };

    let mut master_reader = unsafe { tokio::fs::File::from_raw_fd(pty.fd) };
    let mut master_writer = unsafe { tokio::fs::File::from_raw_fd(pty.fd) };

    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Channel to coordinate shutdown
    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);

    // Task: PTY Output -> WebSocket
    let tx_out = tx.clone();
    tokio::spawn(async move {
        let mut buffer = [0u8; 1024];
        while let Ok(n) = master_reader.read(&mut buffer).await {
            if n == 0 { break; }
            let text = String::from_utf8_lossy(&buffer[..n]).to_string();
            if ws_sender.send(Message::Text(text)).await.is_err() {
                break;
            }
        }
        let _ = tx_out.send(()).await;
    });

    // Task: WebSocket -> PTY Input
    let tx_in = tx.clone();
    tokio::spawn(async move {
        while let Some(msg) = ws_receiver.recv().await {
            if let Ok(Message::Text(t)) = msg {
                if master_writer.write_all(t.as_bytes()).await.is_err() {
                    break;
                }
            }
        }
        let _ = tx_in.send(()).await;
    });

    // Wait for either task to finish
    let _ = rx.recv().await;
    tracing::info!("PTY session closed for socket");
}
