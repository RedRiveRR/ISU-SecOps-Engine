use axum::{
    Json,
    extract::{Path, State},
    response::sse::{Event, Sse},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;

use crate::cli::DirbruteArgs;
use crate::scanner::{ScanEvent, run_dirbrute_core};

#[derive(Clone)]
pub struct AppState {
    pub streams: Arc<Mutex<HashMap<String, mpsc::Receiver<ScanEvent>>>>,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    pub url: String,
    pub wordlist: Option<String>,
    pub threads: usize,
    pub auto_wordlist: bool,
    pub auto_threads: bool,
    pub crawler: bool,
    pub depth: usize,
}

#[derive(Serialize)]
pub struct ScanResponse {
    pub stream_id: String,
}

pub async fn start_server(port: u16) {
    let state = AppState {
        streams: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = axum::Router::new()
        .route("/api/scan", axum::routing::post(start_scan))
        .route("/api/scan/stream/{id}", axum::routing::get(scan_stream))
        .fallback(axum::routing::get_service(
            tower_http::services::ServeDir::new("ui"),
        ))
        .with_state(state);

    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("[*] Web UI is running!");
    println!("[*] Open http://{} in your browser.", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn start_scan(
    State(state): State<AppState>,
    Json(payload): Json<ScanRequest>,
) -> Result<Json<ScanResponse>, Infallible> {
    let (tx, rx) = mpsc::channel(100);
    let stream_id = Uuid::new_v4().to_string();

    {
        let mut streams = state.streams.lock().await;
        streams.insert(stream_id.clone(), rx);
    }

    let args = DirbruteArgs {
        url: payload.url,
        wordlist: payload.wordlist,
        threads: payload.threads,
        headers: vec![],
        cookie: None,
        auto_wordlist: payload.auto_wordlist,
        auto_threads: payload.auto_threads,
        depth: payload.depth,
        show_logs: true,
        crawler: payload.crawler,
        output: None,
        format: None,
    };

    tokio::spawn(async move {
        let _ = run_dirbrute_core(args, tx).await;
    });

    Ok(Json(ScanResponse { stream_id }))
}

async fn scan_stream(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = {
        let mut streams = state.streams.lock().await;
        streams.remove(&id)
    };

    let (tx_err, rx_final) = mpsc::channel(10);

    let stream = match rx {
        Some(r) => ReceiverStream::new(r),
        None => {
            // Cannot find stream, returning error inside the stream to be async safe
            let _ = tx_err.try_send(ScanEvent::Error {
                message: "Stream not found or already consumed.".to_string(),
            });
            ReceiverStream::new(rx_final)
        }
    };

    let mapped_stream =
        stream.map(|ev| Ok(Event::default().data(serde_json::to_string(&ev).unwrap())));

    Sse::new(mapped_stream).keep_alive(axum::response::sse::KeepAlive::new())
}
