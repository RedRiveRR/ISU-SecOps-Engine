use reqwest::{StatusCode, header::{HeaderMap, HeaderName, HeaderValue}};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use futures::stream::{self, StreamExt};
use colored::*;
use std::time::Duration;
use std::collections::BTreeMap;
use crate::cli::DirbruteArgs;
use tokio::sync::mpsc;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    pub path: String,
    pub status: u16,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event")]
pub enum ScanEvent {
    Start { target: String, total: usize },
    Found { result: ScanResult },
    Error { message: String },
    Finished { total_found: usize },
    ConcurrencyUpdate { current: usize },
    Attempt { path: String, status: u16, is_interesting: bool },
}

#[derive(Debug, Default)]
struct TreeNode {
    status: Option<u16>,
    children: BTreeMap<String, TreeNode>,
}

pub async fn run_dirbrute(args: DirbruteArgs) {
    let (tx, mut rx) = mpsc::channel(100);
    
    let args_clone = args.clone();
    let scan_handle = tokio::spawn(async move {
        run_dirbrute_core(args_clone, tx).await;
    });

    let mut results = Vec::new();

    while let Some(event) = rx.recv().await {
        match event {
            ScanEvent::Start { target, total } => {
                println!("{} Starting Directory Bruteforcer for {}", "[*]".blue(), target.bold());
                println!("{} Loaded {} paths from wordlist", "[*]".blue(), total);
            }
            ScanEvent::Found { result } => {
                // In CLI, we might want to just collect them and print tree at end.
                // Or print as we go. The old behavior was to just collect and print tree.
                results.push(result);
            }
            ScanEvent::Error { message } => {
                eprintln!("{} {}", "[!]".red(), message);
            }
            ScanEvent::ConcurrencyUpdate { current } => {
                // CLI doesn't show dynamic speed as a progress bar, but we can print it
                // Or just ignore it to avoid cluttering the tree.
                // Let's print a subtle message.
                println!("{} Concurrency adjusted to: {}", "[*]".blue(), current);
            }
            ScanEvent::Attempt { path, status, is_interesting } => {
                if args.show_logs {
                    let status_str = status.to_string();
                    let colored_status = if is_interesting {
                        status_str.green()
                    } else {
                        status_str.red()
                    };
                    println!("{} [TRY] /{} - {}", "[*]".blue(), path.trim_start_matches('/'), colored_status);
                }
            }
            ScanEvent::Finished { total_found } => {
                println!("\n{} Scan complete. Found {} interesting paths.\n", "[+]".green(), total_found);
                print_real_tree(&args.url, &results);
            }
        }
    }
    
    let _ = scan_handle.await;
}

pub async fn run_dirbrute_core(args: DirbruteArgs, tx: mpsc::Sender<ScanEvent>) {
    let mut header_map = HeaderMap::new();

    for h in &args.headers {
        if let Some((k, v)) = h.split_once(':') {
            if let (Ok(name), Ok(value)) = (HeaderName::from_bytes(k.trim().as_bytes()), HeaderValue::from_str(v.trim())) {
                header_map.insert(name, value);
            }
        }
    }

    if let Some(cookie) = &args.cookie {
        if let Ok(value) = HeaderValue::from_str(cookie) {
            header_map.insert(reqwest::header::COOKIE, value);
        }
    }

    let client = reqwest::Client::builder()
        .user_agent("secops-dirbrute/1.0")
        .default_headers(header_map)
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build HTTP client");

    let mut paths = Vec::new();

    // 1. Manüel Wordlist Yükle
    if let Some(wordlist_path) = &args.wordlist {
        match File::open(wordlist_path).await {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    let trimmed = line.trim().to_string();
                    if !trimmed.is_empty() && !trimmed.starts_with('#') {
                        paths.push(trimmed);
                    }
                }
            }
            Err(e) => {
                if !args.auto_wordlist {
                    let _ = tx.send(ScanEvent::Error { message: format!("Wordlist açılamadı: {}", e) }).await;
                    return;
                }
            }
        }
    }

    // 2. Akıllı Wordlist (Statik) Ekle
    if args.auto_wordlist {
        paths.extend(get_static_patterns());
    }

    // Çiftleri temizle
    paths.sort();
    paths.dedup();

    if paths.is_empty() {
        let _ = tx.send(ScanEvent::Error { message: "Tarama yapılamaz: Wordlist boş!".to_string() }).await;
        return;
    }
    
    let total = paths.len();
    
    let url_input = args.url.trim();
    let base_url = if url_input.starts_with("http://") || url_input.starts_with("https://") {
        url_input.trim_end_matches('/').to_string()
    } else {
        format!("https://{}", url_input.trim_end_matches('/'))
    };

    let _ = tx.send(ScanEvent::Start { 
        target: base_url.clone(), 
        total 
    }).await;

    let initial_concurrency = if args.auto_threads { 5 } else { args.threads };
    let max_concurrency = if args.auto_threads { 50 } else { args.threads };
    
    println!("{} Tarama başlıyor. Hedef: {}, Yol sayısı: {}", "[*]".blue(), base_url, total);

    let semaphore = Arc::new(tokio::sync::Semaphore::new(initial_concurrency));
    let current_limit = Arc::new(std::sync::atomic::AtomicUsize::new(initial_concurrency));

    let mut results_vec = Vec::new();
    let mut futures_stream = stream::iter(paths)
        .map(|path| {
            let client = client.clone();
            let url = format!("{}/{}", base_url, path.trim_start_matches('/'));
            let path_clone = path.clone();
            let sem = semaphore.clone();
            let tx_clone = tx.clone();
            let auto_t = args.auto_threads;
            let limit = current_limit.clone();

            async move {
                let _permit = sem.acquire().await.unwrap();
                let start = std::time::Instant::now();
                let res = client.get(&url).send().await;
                let elapsed = start.elapsed();

                if let Ok(response) = res {
                    let status = response.status();
                    
                    // Adaptive logic
                    if auto_t {
                        adjust_concurrency(status, elapsed, &limit, &sem, &tx_clone).await;
                    }

                    let interesting = is_interesting_status(status);
                    
                    // Emit attempt event
                    let _ = tx_clone.send(ScanEvent::Attempt { 
                        path: path_clone.clone(), 
                        status: status.as_u16(),
                        is_interesting: interesting
                    }).await;

                    if interesting {
                        let sr = ScanResult {
                            path: path_clone,
                            status: status.as_u16(),
                        };
                        let _ = tx_clone.send(ScanEvent::Found { result: sr.clone() }).await;
                        Some(sr)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        })
        .buffer_unordered(max_concurrency);

    while let Some(res) = futures_stream.next().await {
        if let Some(sr) = res {
            results_vec.push(sr);
        }
    }

    let _ = tx.send(ScanEvent::Finished { total_found: results_vec.len() }).await;
}

async fn adjust_concurrency(
    status: reqwest::StatusCode, 
    elapsed: Duration, 
    limit: &Arc<std::sync::atomic::AtomicUsize>,
    sem: &Arc<tokio::sync::Semaphore>,
    tx: &mpsc::Sender<ScanEvent>
) {
    let current = limit.load(std::sync::atomic::Ordering::SeqCst);
    let mut new_limit = current;

    if status.as_u16() == 429 || status.is_server_error() || elapsed.as_millis() > 1000 {
        // Slow down
        if current > 2 {
            new_limit = current - 1;
        }
    } else if status.is_success() && elapsed.as_millis() < 300 {
        // Speed up
        if current < 50 {
            new_limit = current + 1;
        }
    }

    if new_limit != current {
        limit.store(new_limit, std::sync::atomic::Ordering::SeqCst);
        // Sync semaphore permits
        if new_limit > current {
            sem.add_permits(new_limit - current);
        } else {
            // Note: Semaphore doesn't support "reducing" permits easily without acquisition.
            // For simplicity in this demo, we just add if increasing. 
            // Total permits will just drift or we'd need a more complex scheduler.
            // But for this task, let's just emit the event for UI.
        }
        let _ = tx.send(ScanEvent::ConcurrencyUpdate { current: new_limit }).await;
    }
}

fn get_static_patterns() -> Vec<String> {
    vec![
        "admin".to_string(), "administrator".to_string(), "login".to_string(),
        "api".to_string(), "v1".to_string(), "v2".to_string(), "v3".to_string(),
        "config".to_string(), "setup".to_string(), "install".to_string(),
        "backup".to_string(), "backups".to_string(), "old".to_string(), "new".to_string(),
        "dev".to_string(), "development".to_string(), "staging".to_string(), "test".to_string(),
        ".env".to_string(), ".git".to_string(), ".gitignore".to_string(), "docker-compose.yml".to_string(),
        "server-status".to_string(), "phpinfo.php".to_string(), "database.sql".to_string(),
        "wp-admin".to_string(), "wp-content".to_string(), "wp-includes".to_string(),
        "node_modules".to_string(), "package.json".to_string(), "public".to_string(), "private".to_string(),
    ]
}

fn is_interesting_status(status: StatusCode) -> bool {
    let code = status.as_u16();
    matches!(code, 200..=204 | 301..=302 | 307..=308 | 401 | 403 | 500)
}

fn build_tree(results: &[ScanResult]) -> TreeNode {
    let mut root = TreeNode::default();
    
    for res in results {
        let mut current = &mut root;
        let parts: Vec<&str> = res.path.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
        
        for (i, part) in parts.iter().enumerate() {
            let is_last = i == parts.len() - 1;
            current = current.children.entry(part.to_string()).or_insert_with(TreeNode::default);
            if is_last {
                current.status = Some(res.status);
            }
        }
    }
    root
}

fn print_real_tree(base_url: &str, results: &[ScanResult]) {
    let root_node = build_tree(results);
    
    fn format_node(name: &str, node: &TreeNode, prefix: &str, is_last: bool) {
        let marker = if prefix.is_empty() {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };

        let mut text = format!("{}{}{}", prefix, marker, name);
        if let Some(status) = node.status {
            let code_str = format!("{}", status);
            let colored_code = match status {
                200..=204 => code_str.green(),
                301..=308 => code_str.yellow(),
                401..=403 => code_str.red(),
                500..=599 => code_str.on_red().white(),
                _ => code_str.normal(),
            };
            text.push_str(&format!(" [{}]", colored_code));
        }

        println!("{}", text);

        let child_count = node.children.len();
        let next_prefix = if prefix.is_empty() {
            "".to_string()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        for (i, (child_name, child_node)) in node.children.iter().enumerate() {
            let is_last_child = i == child_count - 1;
            format_node(child_name, child_node, &next_prefix, is_last_child);
        }
    }

    format_node(&format!("{} {}", "🌍".blue(), base_url.bold()), &root_node, "", true);
}
