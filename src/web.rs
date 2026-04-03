use axum::{
    extract::{Path, State},
    response::{Html, sse::{Event, Sse}},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::{Mutex, mpsc};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use crate::cli::DirbruteArgs;
use crate::scanner::{run_dirbrute_core, ScanEvent};

static INDEX_HTML: &str = include_str!("../ui/index.html");
static STREAM_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone)]
pub struct AppState {
    streams: Arc<Mutex<HashMap<String, mpsc::Receiver<ScanEvent>>>>,
}

#[derive(Deserialize)]
pub struct ScanRequest {
    url: String,
    wordlist: Option<String>,
    threads: Option<usize>,
    auto_wordlist: Option<bool>,
    auto_threads: Option<bool>,
}

#[derive(Serialize)]
pub struct ScanResponse {
    stream_id: String,
}

pub async fn start_server(port: u16) {
    let state = AppState {
        streams: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/", get(serve_ui))
        .route("/api/scan", post(start_scan))
        .route("/api/scan/stream/{id}", get(scan_stream))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
        
    println!("[*] Web UI is running!");
    println!("[*] Open http://127.0.0.1:{} in your browser.", port);
    
    axum::serve(listener, app).await.unwrap();
}

async fn serve_ui() -> Html<&'static str> {
    Html(INDEX_HTML)
}

async fn start_scan(
    State(state): State<AppState>,
    Json(payload): Json<ScanRequest>,
) -> Result<Json<ScanResponse>, axum::http::StatusCode> {
    
    let args = DirbruteArgs {
        url: payload.url,
        wordlist: payload.wordlist,
        threads: payload.threads.unwrap_or(10),
        headers: vec![],
        cookie: None,
        auto_wordlist: payload.auto_wordlist.unwrap_or(false),
        auto_threads: payload.auto_threads.unwrap_or(false),
    };

    let (tx, rx) = mpsc::channel(100);

    // Spawn scanner in background
    tokio::spawn(async move {
        run_dirbrute_core(args, tx).await;
    });

    let stream_id = format!("scan_{}", STREAM_ID_COUNTER.fetch_add(1, Ordering::SeqCst));
    
    let mut streams = state.streams.lock().await;
    streams.insert(stream_id.clone(), rx);

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

    let stream = match rx {
        Some(r) => ReceiverStream::new(r),
        None => {
            // Cannot find stream, returning immediate finish event
            let (tx, empty_rx) = mpsc::channel(1);
            let _ = tx.blocking_send(ScanEvent::Error { message: "Stream not found or already consumed.".to_string() });
            ReceiverStream::new(empty_rx)
        }
    };

    let mapped_stream = stream.map(|ev| {
        Ok(Event::default().data(serde_json::to_string(&ev).unwrap()))
    });

    Sse::new(mapped_stream).keep_alive(axum::response::sse::KeepAlive::new())
}
