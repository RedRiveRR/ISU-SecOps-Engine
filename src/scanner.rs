use reqwest::{StatusCode, header::{HeaderMap, HeaderName, HeaderValue}};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use futures::stream::{self, StreamExt};
use colored::*;
use std::time::Duration;
use std::collections::BTreeMap;
use crate::cli::DirbruteArgs;

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub path: String,
    pub status: StatusCode,
}

#[derive(Debug, Default)]
struct TreeNode {
    status: Option<StatusCode>,
    children: BTreeMap<String, TreeNode>,
}

pub async fn run_dirbrute(args: DirbruteArgs) {
    println!("{} Starting Directory Bruteforcer for {}", "[*]".blue(), args.url.bold());
    println!("{} Wordlist: {}, Threads: {}", "[*]".blue(), args.wordlist, args.threads);

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

    let file = match File::open(&args.wordlist).await {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} Failed to open wordlist: {}", "[!]".red(), e);
            return;
        }
    };
    
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut paths = Vec::new();
    
    while let Ok(Some(line)) = lines.next_line().await {
        let trimmed = line.trim().to_string();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            paths.push(trimmed);
        }
    }
    
    let total = paths.len();
    println!("{} Loaded {} paths from wordlist", "[*]".blue(), total);

    let base_url = args.url.trim_end_matches('/');

    let results = stream::iter(paths)
        .map(|path| {
            let client = client.clone();
            // Handle cases where wordlist item starts with / or not
            let url = format!("{}/{}", base_url, path.trim_start_matches('/'));
            let path_clone = path.clone();
            tokio::spawn(async move {
                let res = client.get(&url).send().await;
                (path_clone, res)
            })
        })
        .buffer_unordered(args.threads)
        .filter_map(|res| async {
            if let Ok((original_path, Ok(response))) = res {
                let status = response.status();
                if is_interesting_status(status) {
                    Some(ScanResult {
                        path: original_path,
                        status,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .await;

    println!("\n{} Scan complete. Found {} interesting paths.\n", "[+]".green(), results.len());

    print_real_tree(&args.url, &results);
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
            let code_str = format!("{}", status.as_u16());
            let colored_code = match status.as_u16() {
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
