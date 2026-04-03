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
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    pub path: String,
    pub status: u16,
    pub length: u64,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event")]
pub enum ScanEvent {
    Start {
        target: String,
        total: usize,
    },
    Found {
        result: ScanResult,
    },
    Error {
        message: String,
    },
    Finished {
        total_found: usize,
    },
    ConcurrencyUpdate {
        current: usize,
    },
    Attempt {
        path: String,
        status: u16,
        is_interesting: bool,
    },
    CrawlFound {
        path: String,
        source: String,
    },
    WafWarning {
        message: String,
    },
    TechFound {
        name: String,
    },
    Soft404 {
        status: u16,
        length: u64,
    },
}

#[derive(Debug, Default)]
struct TreeNode {
    status: Option<u16>,
    length: Option<u64>,
    title: Option<String>,
    children: BTreeMap<String, TreeNode>,
}

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
                    "{} Starting Directory Bruteforcer for {}",
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
            ScanEvent::CrawlFound { path, source } => {
                if args.show_logs {
                    println!(
                        "{} [CRAWL] Discovered /{} (linked from /{})",
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
            ) {
                header_map.insert(name, value);
            }
    }

    if let Some(cookie) = &args.cookie
        && let Ok(value) = HeaderValue::from_str(cookie) {
            header_map.insert(reqwest::header::COOKIE, value);
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
            Err(_) => {
                if !args.auto_wordlist {
                    let _ = tx
                        .send(ScanEvent::Error {
                            message: "Wordlist açılamadı".into(),
                        })
                        .await;
                    return Ok(vec![]);
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
    let random_path = "ISU-SecOps-Heuristic-404-Test";
    if let Ok(resp) = client
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
                    let client = client.clone();
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

                    tokio::spawn(async move {
                        let _permit = sem.acquire().await.unwrap();
                        let start = std::time::Instant::now();
                        let url = format!("{}/{}", base_url_clone, path_clone);
                        let res = client.get(&url).send().await;
                        let elapsed = start.elapsed();

                        let mut found_result = None;

                        if let Ok(response) = res {
                            let status = response.status();

                            if status.as_u16() == 403 || status.as_u16() == 429 {
                                let old = waf_error_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                                if old == 9 && !waf_warned.swap(true, std::sync::atomic::Ordering::SeqCst) {
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
                                    && status.as_u16() == base_status && length == base_len {
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
                                if args_crawl && depth < max_depth
                                    && let Some(body) = body_text {
                                        let links = extract_links(&body);
                                        let mut v = visited_clone.lock().await;
                                        for link in links {
                                            if let Some(normalized) = normalize_path(&link, &base_url_clone)
                                                && v.insert(normalized.clone()) {
                                                    let _ = tx_clone.send(ScanEvent::CrawlFound {
                                                        path: normalized.clone(),
                                                        source: path_clone.clone()
                                                    }).await;
                                                    let _ = master_tx_clone.send((normalized, depth + 1)).await;
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
    } else if status.is_success() && elapsed.as_millis() < 300
        && current < 50 {
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

fn get_static_patterns() -> Vec<String> {
    vec![
        "admin".into(),
        "administrator".into(),
        "login".into(),
        "api".into(),
        "v1".into(),
        "v2".into(),
        "v3".into(),
        "config".into(),
        "setup".into(),
        "install".into(),
        "backup".into(),
        "backups".into(),
        "old".into(),
        "new".into(),
        "dev".into(),
        "development".into(),
        "staging".into(),
        "test".into(),
        ".env".into(),
        ".git".into(),
        ".gitignore".into(),
        "docker-compose.yml".into(),
        "server-status".into(),
        "phpinfo.php".into(),
        "database.sql".into(),
        "wp-admin".into(),
        "wp-content".into(),
        "wp-includes".into(),
        "node_modules".into(),
        "package.json".into(),
        "public".into(),
        "private".into(),
    ]
}

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

fn extract_links(html: &str) -> Vec<String> {
    let mut links = Vec::new();
    let doc = Document::from(html);
    for el in doc.select("a").iter() {
        if let Some(href) = el.attr("href") {
            links.push(href.to_string());
        }
    }
    links
}

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
