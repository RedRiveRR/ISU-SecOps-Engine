use crate::cli::DirbruteArgs;
use colored::*;
use nipper::Document;
use reqwest::{
    StatusCode,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;
use tokio::sync::mpsc;

/// Represents a single discovery result from the scanner.
#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    /// The discovered path (e.g. /admin).
    pub path: String,
    /// HTTP status code (e.g. 200, 301, 403).
    pub status: u16,
    /// Content length in bytes.
    pub length: u64,
    /// Page title extracted from the HTML <title> tag.
    pub title: Option<String>,
}

/// Enumeration of events emitted by the scanner during its lifecycle.
/// Used for real-time communication with CLI and Web UI.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event")]
pub enum ScanEvent {
    /// Dispatched when the scan starts.
    Start {
        /// Fully qualified target URL.
        target: String,
        /// Total number of paths to be tested.
        total: usize,
    },
    /// Dispatched when a relevant directory or file is found.
    Found {
        /// The result data.
        result: ScanResult,
    },
    /// Dispatched when a critical error occurs (e.g. network lost).
    Error {
        /// The error message.
        message: String,
    },
    /// Dispatched when the entire scan process completes.
    Finished {
        /// Total number of interesting results discovered.
        total_found: usize,
    },
    /// Dispatched when adaptive threading changes the concurrency level.
    ConcurrencyUpdate {
        /// New number of concurrent workers.
        current: usize,
    },
    /// Dispatched for every request attempt (for live logging).
    Attempt {
        /// The path being tested.
        path: String,
        /// HTTP status code received.
        status: u16,
        /// Whether the result is considered "interesting" (non-404).
        is_interesting: bool,
    },
    /// Dispatched when the HTML Crawler finds a new internal link.
    CrawlFound {
        /// The discovered path.
        path: String,
        /// Where the link was found (source page).
        source: String,
    },
    /// Dispatched when a WAF (Web App Firewall) detection occurs.
    WafWarning {
        /// Descriptive warning message.
        message: String,
    },
    /// Dispatched when a specific technology (e.g. WordPress) is detected.
    TechFound {
        /// Name of the detected technology.
        name: String,
    },
    /// Dispatched when a Soft-404 false positive is identified.
    Soft404 {
        /// HTTP status code returned (usually 200).
        status: u16,
        /// Body length used for filtering.
        length: u64,
    },
    /// Status update for the Deep Stealth system.
    StealthStatus {
        /// Information message (e.g. "Starting cool-down").
        message: String,
    },
}

#[derive(Debug, Default)]
struct TreeNode {
    status: Option<u16>,
    length: Option<u64>,
    title: Option<String>,
    children: BTreeMap<String, TreeNode>,
}

/// Public Entry Point: Runs the directory bruteforcer with the provided arguments.
/// Manages both the CLI output and the async execution core.
pub async fn run_dirbrute(args: DirbruteArgs) {
    let (tx, mut rx) = mpsc::channel(5000);

    let args_clone = args.clone();
    let scan_handle = tokio::spawn(async move {
        let _ = run_dirbrute_core(args_clone, tx).await;
    });

    let mut results = Vec::new();

    while let Some(event) = rx.recv().await {
        match event {
            ScanEvent::Start { target, total } => {
                println!(
                    "{} Starting Directory Bruteforcer (dirbrute v1.1.0) for {}",
                    "[*]".blue(),
                    target.bold()
                );
                println!("{} Loaded {} paths from wordlist", "[*]".blue(), total);
            }
            ScanEvent::Found { result } => {
                results.push(result);
            }
            ScanEvent::Error { message } => {
                eprintln!("{} {}", "[!]".red(), message);
            }
            ScanEvent::ConcurrencyUpdate { current } => {
                println!("{} Concurrency adjusted to: {}", "[*]".blue(), current);
            }
            ScanEvent::Attempt {
                path,
                status,
                is_interesting,
            } => {
                if args.show_logs {
                    let status_str = status.to_string();
                    let colored_status = if is_interesting {
                        status_str.green()
                    } else {
                        status_str.red()
                    };
                    println!(
                        "{} [TRY] /{} - {}",
                        "[*]".blue(),
                        path.trim_start_matches('/'),
                        colored_status
                    );
                }
            }
            ScanEvent::WafWarning { message } => {
                println!(
                    "\n{} {}\n",
                    "[!] WAF TESPİT EDİLDİ:".on_red().white().bold(),
                    message.yellow().bold()
                );
            }
            ScanEvent::TechFound { name } => {
                if args.show_logs {
                    println!(
                        "{} Teşhis Edilen Teknoloji: {}",
                        "[🏷️]".magenta(),
                        name.bold()
                    );
                }
            }
            ScanEvent::Soft404 { status, length } => {
                println!(
                    "{} Soft-404 Koruma Aktif. Baseline: {} ({} bytes)",
                    "[🛡️]".cyan(),
                    status,
                    length
                );
            }
            ScanEvent::StealthStatus { message } => {
                println!("{} {}", "[🥷 STEALTH]".magenta().bold(), message.magenta());
            }
            ScanEvent::CrawlFound { path, source } => {
                if args.show_logs {
                    println!(
                        "{} [CRAWL] Discovered /{} ({})",
                        "[+]".green(),
                        path,
                        source
                    );
                }
            }
            ScanEvent::Finished { total_found } => {
                println!(
                    "\n{} Scan complete. Found {} interesting paths.\n",
                    "[+]".green(),
                    total_found
                );
                print_real_tree(&args.url, &results);

                if let Some(output_path) = &args.output {
                    if let Err(e) = save_results(output_path, &args.format, &results) {
                        eprintln!("{} Error saving results: {}", "[!]".red(), e);
                    } else {
                        println!("{} Results saved to {}", "[+]".green(), output_path.bold());
                    }
                }
            }
        }
    }

    let _ = scan_handle.await;
}

/// Core execution engine for the directory bruteforcer.
/// 
/// # Architecture
/// This function implements a highly concurrent, asynchronous polling engine using 
/// `tokio`'s MPSC channels and Semaphores. It integrates several advanced security 
/// assessment modules:
/// 
/// 1. **Adaptive Concurrency**: Automatically scales worker threads based on server latency.
/// 2. **Deep Stealth**: Implements contextual blending and autonomous cool-down periods.
/// 3. **Heuristic Detection**: Filters 'Soft-404' responses using baseline profiling.
/// 4. **Dynamic Crawler**: Recursively extracts and processes links from HTML/JS/CSS.
///
/// # Academic Implementation Note
/// The engine is designed with a "Safety-First" approach, ensuring that even under 
/// high concurrency, the target server is not overwhelmed (DoS protection) through 
/// the use of `tokio::sync::Semaphore`.
pub async fn run_dirbrute_core(
    args: DirbruteArgs,
    tx: mpsc::Sender<ScanEvent>,
) -> Result<Vec<ScanResult>, Box<dyn std::error::Error + Send + Sync>> {
    let mut header_map = HeaderMap::new();

    for h in &args.headers {
        if let Some((k, v)) = h.split_once(':')
            && let (Ok(name), Ok(value)) = (
                HeaderName::from_bytes(k.trim().as_bytes()),
                HeaderValue::from_str(v.trim()),
            )
        {
            header_map.insert(name, value);
        }
    }

    if let Some(cookie) = &args.cookie
        && let Ok(value) = HeaderValue::from_str(cookie)
    {
        header_map.insert(reqwest::header::COOKIE, value);
    }

    let mut clients = Vec::new();
    if let Some(proxy_urls) = &args.proxy {
        for proxy_url in proxy_urls.split(',') {
            let proxy_url_t = proxy_url.trim();
            if proxy_url_t.is_empty() {
                continue;
            }
            let mut builder = reqwest::Client::builder()
                .user_agent("dirbrute/1.1.0")
                .default_headers(header_map.clone())
                .redirect(reqwest::redirect::Policy::none())
                .timeout(Duration::from_secs(10));

            if let Ok(proxy) = reqwest::Proxy::all(proxy_url_t) {
                builder = builder.proxy(proxy);
            }
            if let Ok(client) = builder.build() {
                clients.push(client);
            }
        }
    }

    if clients.is_empty() {
        let builder = reqwest::Client::builder()
            .user_agent("dirbrute/1.1.0")
            .default_headers(header_map.clone())
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(10));

        match builder.build() {
            Ok(client) => clients.push(client),
            Err(e) => {
                let _ = tx
                    .send(ScanEvent::Error {
                        message: format!("HTTP Client could not be initialized: {}", e),
                    })
                    .await;
                return Ok(vec![]);
            }
        }
    }

    let clients_arc = Arc::new(clients);

    let mut paths = Vec::new();

    // 1. Manüel Wordlist Yükle
    if let Some(wordlist_path) = &args.wordlist {
        let path = Path::new(wordlist_path);
        let final_path = if path.exists() {
            Some(path.to_path_buf())
        } else {
            let alt_path = Path::new("wordlists").join(wordlist_path);
            if alt_path.exists() {
                Some(alt_path)
            } else {
                None
            }
        };

        match final_path {
            Some(p) => {
                if let Ok(file) = File::open(p).await {
                    let reader = BufReader::new(file);
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        let trimmed = line.trim().to_string();
                        if !trimmed.is_empty() && !trimmed.starts_with('#') {
                            paths.push(trimmed);
                        }
                    }
                }
            }
            None => {
                if !args.auto_wordlist {
                    let _ = tx
                        .send(ScanEvent::Error {
                            message: format!("Wordlist bulunamadı: {}", wordlist_path),
                        })
                        .await;
                    return Ok(vec![]);
                } else {
                    let _ = tx
                        .send(ScanEvent::StealthStatus {
                            message: format!("Wordlist ({}) bulunamadı, dahili patenlerle devam ediliyor.", wordlist_path),
                        })
                        .await;
                }
            }
        }
    }

    if args.auto_wordlist {
        paths.extend(get_static_patterns());
    }

    paths.sort();
    paths.dedup();

    let total = paths.len();
    let url_input = args.url.clone();
    let base_url = if url_input.starts_with("http://") || url_input.starts_with("https://") {
        url_input.trim_end_matches('/').to_string()
    } else {
        format!("https://{}", url_input.trim_end_matches('/'))
    };

    let _ = tx
        .send(ScanEvent::Start {
            target: base_url.clone(),
            total,
        })
        .await;

    let initial_concurrency = if args.auto_threads { 5 } else { args.threads };
    let max_concurrency = if args.auto_threads { 50 } else { args.threads };

    // --- Soft-404 Calibration ---
    let mut soft_404_baseline: Option<(u16, u64)> = None;
    let random_path = "DirBrute-Heuristic-404-Test";
    if let Ok(resp) = clients_arc[0]
        .get(format!("{}/{}", base_url, random_path))
        .send()
        .await
    {
        let s = resp.status().as_u16();
        let l = if let Some(content_len) = resp.content_length() {
            content_len
        } else {
            resp.text().await.unwrap_or_default().len() as u64
        };

        if s == 200 || s == 301 || s == 302 {
            soft_404_baseline = Some((s, l));
            let _ = tx
                .send(ScanEvent::Soft404 {
                    status: s,
                    length: l,
                })
                .await;
        }
    }

    let semaphore = Arc::new(tokio::sync::Semaphore::new(initial_concurrency));
    let current_limit = Arc::new(std::sync::atomic::AtomicUsize::new(initial_concurrency));
    let waf_error_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let waf_warned = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let cooldown_until = Arc::new(std::sync::atomic::AtomicU64::new(0));

    let visited = Arc::new(Mutex::new(HashSet::new()));
    let technologies_found = Arc::new(Mutex::new(HashSet::new()));
    let (master_tx, mut queue_rx) = mpsc::channel::<(String, usize)>(10000);
    let (done_tx, mut done_rx) = mpsc::channel::<(Option<ScanResult>, String)>(10000);
    let mut results_vec = Vec::new();
    let mut active_tasks = 0;
    let soft_404_baseline_arc = Arc::new(soft_404_baseline);

    let paths_arc = Arc::new(paths);
    {
        let mut v = visited.lock().await;
        for path in paths_arc.iter() {
            let normalized = path.trim_start_matches('/').to_string();
            if !normalized.is_empty() && v.insert(normalized.clone()) {
                let _ = master_tx.send((normalized, 0)).await;
            }
        }
    }

    loop {
        if active_tasks == 0 && queue_rx.is_empty() {
            break;
        }

        tokio::select! {
            res = queue_rx.recv(), if active_tasks < max_concurrency => {
                if let Some((path, depth)) = res {
                    active_tasks += 1;
                    let clients = clients_arc.clone();
                    let base_url_clone = base_url.clone();
                    let path_clone = path.clone();
                    let sem = semaphore.clone();
                    let tx_clone = tx.clone();
                    let master_tx_clone = master_tx.clone();
                    let done_tx_clone = done_tx.clone();
                    let visited_clone = visited.clone();
                    let limit = current_limit.clone();
                    let max_depth = args.depth;
                    let technologies_found_clone = technologies_found.clone();
                    let waf_error_count = waf_error_count.clone();
                    let waf_warned = waf_warned.clone();
                    let soft_404_baseline = soft_404_baseline_arc.clone();
                    let args_crawl = args.crawler;
                    let auto_threads = args.auto_threads;
                    let paths_arc_clone = paths_arc.clone();
                    let stealth_mode = args.stealth;
                    let cooldown_until_clone = cooldown_until.clone();

                    tokio::spawn(async move {
                        let _permit = sem.acquire().await.unwrap();

                        if stealth_mode {
                            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                            let cd = cooldown_until_clone.load(std::sync::atomic::Ordering::SeqCst);
                            if now < cd {
                                tokio::time::sleep(tokio::time::Duration::from_secs(cd - now)).await;
                            }
                        }

                        let mut decoy_client_idx = 0;
                        let mut main_client_idx = 0;

                        let (should_decoy, decoy_index, jitter, random_ip, random_ua_idx) = {
                            use rand::Rng;
                            let mut rng = rand::thread_rng();
                            let ip = format!("{}.{}.{}.{}", rng.gen_range(1..255), rng.gen_range(1..255), rng.gen_range(1..255), rng.gen_range(1..=254));
                            if !clients.is_empty() {
                                decoy_client_idx = rng.gen_range(0..clients.len());
                                main_client_idx = rng.gen_range(0..clients.len());
                            }
                            (
                                rng.gen_bool(0.15),
                                rng.gen_range(0..6),
                                rng.gen_range(10..150),
                                ip,
                                rng.gen_range(0..6)
                            )
                        };

                        if stealth_mode && should_decoy {
                            let decoys = ["", "favicon.ico", "robots.txt", "sitemap.xml", "assets/", "public/"];
                            let decoy = decoys[decoy_index];
                            let decoy_url = format!("{}/{}", base_url_clone, decoy);
                            let _ = tx_clone.send(ScanEvent::StealthStatus { message: format!("Contextual blending: fetched /{}", decoy) }).await;
                            let _ = clients[decoy_client_idx].get(&decoy_url).send().await;
                        }

                        // Jitter to evade simple timing-based heuristics
                        tokio::time::sleep(tokio::time::Duration::from_millis(jitter)).await;

                        let user_agents = [
                            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
                            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
                            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:123.0) Gecko/20100101 Firefox/123.0",
                            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
                            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_3_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.3 Mobile/15E148 Safari/604.1",
                            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Edge/122.0.0.0 Safari/537.36",
                        ];
                        let random_ua = user_agents[random_ua_idx];

                        let start = std::time::Instant::now();
                        let url = format!("{}/{}", base_url_clone, path_clone);

                        let res = clients[main_client_idx].get(&url)
                            .header("User-Agent", random_ua)
                            .header("X-Forwarded-For", &random_ip)
                            .header("X-Originating-IP", &random_ip)
                            .header("X-Remote-IP", &random_ip)
                            .header("X-Remote-Addr", &random_ip)
                            .header("X-Real-IP", &random_ip)
                            .header("Client-IP", &random_ip)
                            .header("True-Client-IP", &random_ip)
                            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
                            .header("Accept-Language", "en-US,en;q=0.5")
                            .header("Upgrade-Insecure-Requests", "1")
                            .send().await;

                        let elapsed = start.elapsed();

                        let mut found_result = None;

                        if let Ok(response) = res {
                            let status = response.status();

                            if status.as_u16() == 403 || status.as_u16() == 429 {
                                let old = waf_error_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                                if stealth_mode && old >= 3 {
                                    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                                    let cd = cooldown_until_clone.load(std::sync::atomic::Ordering::SeqCst);
                                    if now >= cd {
                                        cooldown_until_clone.store(now + 60, std::sync::atomic::Ordering::SeqCst);
                                        waf_error_count.store(0, std::sync::atomic::Ordering::SeqCst);
                                        let _ = tx_clone.send(ScanEvent::StealthStatus {
                                            message: "WAF Detected. Cooling down for 60 seconds...".into()
                                        }).await;
                                    }
                                } else if old == 9 && !waf_warned.swap(true, std::sync::atomic::Ordering::SeqCst) {
                                    let _ = tx_clone.send(ScanEvent::WafWarning {
                                        message: "Yüksek oranda 403/429 hatası! WAF tespit edildi.".into()
                                    }).await;
                                }
                            } else if status.is_success() {
                                waf_error_count.store(0, std::sync::atomic::Ordering::SeqCst);
                            }

                            if auto_threads {
                                adjust_concurrency(status, elapsed, &limit, &sem, &tx_clone).await;
                            }

                            let interesting = is_interesting_status(status);

                            if interesting {
                                let lower_path = path_clone.to_lowercase();
                                for (key, tech) in get_fingerprints() {
                                    if lower_path.contains(key) {
                                        let mut techs = technologies_found_clone.lock().await;
                                        if techs.insert(tech.to_string()) {
                                            let _ = tx_clone.send(ScanEvent::TechFound { name: tech.to_string() }).await;
                                        }
                                    }
                                }
                            }

                            let mut is_truly_interesting = interesting;
                            let content_type = response.headers().get(reqwest::header::CONTENT_TYPE)
                                .and_then(|v| v.to_str().ok())
                                .unwrap_or("")
                                .to_string();

                            let mut length = response.content_length().unwrap_or(0);
                            let mut title = None;
                            let is_html = content_type.contains("text/html");

                            let needs_body = is_html || length == 0;
                            let body_text = if needs_body {
                                response.text().await.ok()
                            } else { None };

                            if let Some(ref text) = body_text {
                                if length == 0 { length = text.len() as u64; }
                                if is_html {
                                    let doc = Document::from(text.as_str());
                                    let t = doc.select("title").text().to_string();
                                    if !t.trim().is_empty() {
                                        title = Some(t.trim().to_string());
                                    }
                                }
                            }

                            if is_truly_interesting
                                && let Some((base_status, base_len)) = *soft_404_baseline
                                && status.as_u16() == base_status
                                && length == base_len
                            {
                                is_truly_interesting = false;
                            }

                            let _ = tx_clone.send(ScanEvent::Attempt {
                                path: path_clone.clone(),
                                status: status.as_u16(),
                                is_interesting: is_truly_interesting
                            }).await;

                            if is_truly_interesting {
                                let sr = ScanResult {
                                    path: path_clone.clone(),
                                    status: status.as_u16(),
                                    length,
                                    title,
                                };
                                found_result = Some(sr.clone());
                                let _ = tx_clone.send(ScanEvent::Found { result: sr }).await;

                                // Recursive Discovery
                                if depth < max_depth && !path_clone.contains('.') {
                                    for p in paths_arc_clone.iter() {
                                        let sub_path = format!("{}/{}", path_clone.trim_end_matches('/'), p.trim_start_matches('/'));
                                        let mut v = visited_clone.lock().await;
                                        if v.insert(sub_path.clone()) {
                                            let _ = master_tx_clone.send((sub_path, depth + 1)).await;
                                        }
                                    }
                                }

                                // Crawler
                                if args_crawl && depth < max_depth && let Some(ref body) = body_text {
                                    let findings = extract_metadata(body);
                                    let mut v = visited_clone.lock().await;
                                    for (path, source) in findings {
                                        if let Some(normalized) = normalize_path(&path, &base_url_clone)
                                            && v.insert(normalized.clone())
                                        {
                                            let _ = tx_clone.send(ScanEvent::CrawlFound {
                                                path: normalized,
                                                source
                                            }).await;
                                        }
                                    }
                                }

                                // Suffix Probing (Smart Mode Extra)
                                if args.auto_wordlist && !path_clone.contains('.') {
                                    let suffixes = [".bak", ".old", ".txt", ".env", ".zip", ".tar.gz"];
                                    for suffix in suffixes {
                                        let probed = format!("{}{}", path_clone, suffix);
                                        let mut v = visited_clone.lock().await;
                                        if v.insert(probed.clone()) {
                                            let _ = master_tx_clone.send((probed, 0)).await;
                                        }
                                    }
                                }
                            }
                        }
                        let _ = done_tx_clone.send((found_result, path_clone)).await;
                    });
                }
            }
            Some((res, _path)) = done_rx.recv() => {
                active_tasks -= 1;
                if let Some(sr) = res {
                    results_vec.push(sr);
                }
            }
        }
    }

    let _ = tx
        .send(ScanEvent::Finished {
            total_found: results_vec.len(),
        })
        .await;
    Ok(results_vec)
}

/// Internal logic to handle adaptive concurrency based on server response and latency.
async fn adjust_concurrency(
    status: reqwest::StatusCode,
    elapsed: Duration,
    limit: &Arc<std::sync::atomic::AtomicUsize>,
    sem: &Arc<tokio::sync::Semaphore>,
    tx: &mpsc::Sender<ScanEvent>,
) {
    let current = limit.load(std::sync::atomic::Ordering::SeqCst);
    let mut new_limit = current;

    if status.as_u16() == 429 || status.is_server_error() || elapsed.as_millis() > 1000 {
        if current > 2 {
            new_limit = current - 1;
        }
    } else if status.is_success() && elapsed.as_millis() < 300 && current < 50 {
        new_limit = current + 1;
    }

    if new_limit != current {
        limit.store(new_limit, std::sync::atomic::Ordering::SeqCst);
        if new_limit > current {
            sem.add_permits(new_limit - current);
        }
        let _ = tx
            .send(ScanEvent::ConcurrencyUpdate { current: new_limit })
            .await;
    }
}

/// Returns the built-in static wordlist patterns for smart discovery.
fn get_static_patterns() -> Vec<String> {
    vec![
        "admin".into(),
        "administrator".into(),
        "login".into(),
        "signin".into(),
        "register".into(),
        "signup".into(),
        "api".into(),
        "v1".into(),
        "v2".into(),
        "v3".into(),
        "swagger".into(),
        "docs".into(),
        "api-docs".into(),
        "config".into(),
        "configuration".into(),
        "settings".into(),
        "setup".into(),
        "install".into(),
        "update".into(),
        "backup".into(),
        "backups".into(),
        "old".into(),
        "new".into(),
        "tmp".into(),
        "temp".into(),
        "dump".into(),
        "dev".into(),
        "development".into(),
        "staging".into(),
        "test".into(),
        "testing".into(),
        "demo".into(),
        ".env".into(),
        ".env.local".into(),
        ".env.test".into(),
        ".env.example".into(),
        ".env.production".into(),
        ".git".into(),
        ".git/config".into(),
        ".gitignore".into(),
        ".gitattributes".into(),
        ".ssh".into(),
        "id_rsa".into(),
        "id_dsa".into(),
        "id_ed25519".into(),
        "authorized_keys".into(),
        "credentials".into(),
        "docker-compose.yml".into(),
        "Dockerfile".into(),
        "docker-stack.yml".into(),
        ".docker_env".into(),
        "server-status".into(),
        "phpinfo.php".into(),
        "info.php".into(),
        "status.php".into(),
        "health".into(),
        "database.sql".into(),
        "db.sql".into(),
        "dump.sql".into(),
        "backup.sql".into(),
        "backup.tar.gz".into(),
        "wp-admin".into(),
        "wp-content".into(),
        "wp-includes".into(),
        "wp-config.php".into(),
        "wp-config.php.bak".into(),
        "node_modules".into(),
        "package.json".into(),
        "package-lock.json".into(),
        "yarn.lock".into(),
        "npm-debug.log".into(),
        "public".into(),
        "private".into(),
        "src".into(),
        "app".into(),
        "includes".into(),
        "dist".into(),
        "build".into(),
        ".vscode".into(),
        ".idea".into(),
        "sftp.json".into(),
        ".DS_Store".into(),
        "web.config".into(),
        "htaccess".into(),
        "composer.json".into(),
        "composer.lock".into(),
        "vendor".into(),
        "vendor/autoload.php".into(),
        "bin".into(),
        "scripts".into(),
        "assets".into(),
        "images".into(),
        "uploads".into(),
        "media".into(),
        "auth".into(),
        "authorize".into(),
        "oauth".into(),
        "token".into(),
        "secret".into(),
        "key".into(),
        "aws".into(),
        "s3".into(),
        "bucket".into(),
        "cloud".into(),
        "metadata".into(),
        "internal".into(),
        "manage".into(),
        "manager".into(),
        "control".into(),
        "panel".into(),
        "dashboard".into(),
        "console".into(),
        "debug".into(),
        "trace".into(),
        "log".into(),
        "logs".into(),
        "error.log".into(),
        "access.log".into(),
        "mail".into(),
        "email".into(),
        "smtp".into(),
        "webmail".into(),
        "roundcube".into(),
        "phpmyadmin".into(),
        "robots.txt".into(),
        "sitemap.xml".into(),
        "crossdomain.xml".into(),
        "clientaccesspolicy.xml".into(),
        ".htaccess".into(),
        ".htpasswd".into(),
        ".ssh/id_rsa".into(),
        ".ssh/id_dsa".into(),
        "config/database.php".into(),
        "artisan".into(),
        "manage.py".into(),
        "app.js".into(),
        "main.js".into(),
        "vendor.js".into(),
        "bundle.js".into(),
        "api/health".into(),
        "api/status".into(),
        "api/v1/health".into(),
        "api/v1/status".into(),
        "swagger-ui.html".into(),
        "v2/api-docs".into(),
        "v3/api-docs".into(),
        "swagger.json".into(),
        "graphql".into(),
        "graphiql".into(),
        "playground".into(),
        ".gitlab-ci.yml".into(),
        ".travis.yml".into(),
        "circle.yml".into(),
        "appveyor.yml".into(),
        "Jenkinsfile".into(),
        ".jenkins/config.xml".into(),
    ]
}

/// Returns a list of known file/path signatures for technology fingerprinting.
fn get_fingerprints() -> Vec<(&'static str, &'static str)> {
    vec![
        ("wp-admin", "WordPress"),
        ("wp-content", "WordPress"),
        ("wp-includes", "WordPress"),
        ("node_modules", "Node.js"),
        ("package.json", "Node.js"),
        ("composer.json", "PHP"),
        ("phpinfo.php", "PHP"),
        (".env", "Environment Variables"),
        ("web.config", "IIS / ASP.NET"),
        ("docker-compose.yml", "Docker"),
        ("database.sql", "SQL Database Dump"),
        ("artisan", "Laravel"),
        ("manage.py", "Django"),
        ("app.js", "Node.js Frontend"),
        ("vendor.js", "Frontend Framework"),
        (".jenkins", "Jenkins CI"),
        (".gitlab-ci.yml", "GitLab CI"),
        ("swagger-ui", "Swagger API"),
        ("graphql", "GraphQL API"),
        (".env", "Environment Variables"),
        ("wp-admin", "WordPress"),
    ]
}

/// Determines if an HTTP status code is worth reporting to the user.
fn is_interesting_status(status: StatusCode) -> bool {
    let code = status.as_u16();
    matches!(code, 200..=204 | 301..=302 | 307..=308 | 401 | 403 | 500)
}

/// Internal recursive function to build a hierarchical tree from a flat list of results.
fn build_tree(results: &[ScanResult]) -> TreeNode {
    let mut root = TreeNode::default();
    for res in results {
        let mut current = &mut root;
        let parts: Vec<&str> = res
            .path
            .trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        for (i, part) in parts.iter().enumerate() {
            let is_last = i == parts.len() - 1;
            current = current
                .children
                .entry(part.to_string())
                .or_insert_with(TreeNode::default);
            if is_last {
                current.status = Some(res.status);
                current.length = Some(res.length);
                current.title = res.title.clone();
            }
        }
    }
    root
}

/// Renders the scan results as a professional terminal-based tree.
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
        if let Some(len) = node.length {
            text.push_str(&format!(" (L: {})", len).dimmed().to_string());
        }
        if let Some(ref title) = node.title {
            text.push_str(&format!(" - \"{}\"", title).cyan().to_string());
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
    format_node(
        &format!("{} {}", "🌍".blue(), base_url.bold()),
        &root_node,
        "",
        true,
    );
}

/// Parsers HTML content to extract potential discovery paths (links, scripts, comments).
fn extract_metadata(html: &str) -> Vec<(String, String)> {
    let mut findings = Vec::new();
    let doc = Document::from(html);

    // 1. Tags with paths (Links, Scripts, Images, Forms)
    let selectors = [
        ("a", "href", "Link"),
        ("script", "src", "Script"),
        ("img", "src", "Image"),
        ("link", "href", "Style/Asset"),
        ("form", "action", "Form Action"),
    ];

    for (tag, attr, source) in selectors {
        for el in doc.select(tag).iter() {
            if let Some(val) = el.attr(attr) {
                findings.push((val.to_string(), source.to_string()));
            }
        }
    }

    // 2. HTML Comments
    use regex::Regex;
    let comment_regex = Regex::new(r"(?s)<!--(.*?)-->").unwrap();
    let path_regex = Regex::new(r"(/[a-zA-Z0-9._\-/]+)").unwrap();

    for cap in comment_regex.captures_iter(html) {
        let comment = cap[1].trim();
        if !comment.is_empty() {
            for path_cap in path_regex.captures_iter(comment) {
                findings.push((path_cap[1].to_string(), "Comment".into()));
            }
        }
    }

    findings
}

/// Converts relative paths and absolute-internal URLs into clean, relative paths.
fn normalize_path(path: &str, base_url: &str) -> Option<String> {
    if path.starts_with("http") {
        if path.starts_with(base_url) {
            return Some(
                path.replace(base_url, "")
                    .trim_start_matches('/')
                    .to_string(),
            );
        }
        return None;
    }
    if path.starts_with('/') {
        return Some(path.trim_start_matches('/').to_string());
    }
    if path.starts_with('#') || path.starts_with("javascript:") || path.is_empty() {
        return None;
    }
    Some(path.to_string())
}

/// Serializes and persists scan results to the local filesystem (JSON/CSV).
fn save_results(
    path: &str,
    format: &Option<String>,
    results: &[ScanResult],
) -> Result<(), Box<dyn std::error::Error>> {
    let fmt = format.as_deref().unwrap_or_else(|| {
        if path.ends_with(".csv") {
            "csv"
        } else {
            "json"
        }
    });

    if let Some(parent) = Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    match fmt.to_lowercase().as_str() {
        "csv" => {
            let mut writer = csv::Writer::from_writer(file);
            for res in results {
                writer.serialize(res)?;
            }
            writer.flush()?;
        }
        _ => {
            serde_json::to_writer_pretty(file, results)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_patterns_content() {
        let patterns = get_static_patterns();
        assert!(patterns.contains(&".env".to_string()));
        assert!(patterns.contains(&"wp-admin".to_string()));
        assert!(patterns.len() > 100);
    }

    #[test]
    fn test_scan_result_serialization() {
        let result = ScanResult {
            path: "/admin".to_string(),
            status: 200,
            length: 1234,
            title: Some("Admin Login".to_string()),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"path\":\"/admin\""));
        assert!(json.contains("\"status\":200"));
    }

    #[test]
    fn test_normalize_path() {
        let base = "https://example.com";
        assert_eq!(normalize_path("/admin", base), Some("admin".to_string()));
        assert_eq!(
            normalize_path("config.php", base),
            Some("config.php".to_string())
        );
        assert_eq!(
            normalize_path("https://example.com/internal", base),
            Some("internal".to_string())
        );
        assert_eq!(normalize_path("https://extern.com", base), None);
    }
}
