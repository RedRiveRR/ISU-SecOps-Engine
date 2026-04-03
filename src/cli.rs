use clap::{Parser, Subcommand};

/// Main CLI entry point for the dirbrute tool.
#[derive(Parser, Debug)]
#[command(name = "dirbrute")]
#[command(
    about = "High-performance security assessment and directory discovery engine.",
    version
)]
pub struct Cli {
    /// Subcommand to execute: pentest or web.
    #[command(subcommand)]
    pub command: Command,
}

/// Available subcommands for the security suite.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Launch a penetration testing task (e.g. dirbrute).
    Pentest(PentestArgs),
    /// Start the interactive Web UI.
    Web(WebArgs),
}

/// Arguments for the Web UI mode.
#[derive(Parser, Debug)]
pub struct WebArgs {
    /// Port for the web interface (default: 8080).
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

/// Wrapper for pentest-specific tools.
#[derive(Parser, Debug)]
pub struct PentestArgs {
    /// Choose a tool (dirbrute is currently supported).
    #[command(subcommand)]
    pub pentest_command: PentestCommand,
}

/// Specific tools available under the pentest suite.
#[derive(Subcommand, Debug)]
pub enum PentestCommand {
    /// Directory and file discovery engine.
    Dirbrute(DirbruteArgs),
}

/// Core configuration for the dirbrute directory discovery engine.
#[derive(Parser, Debug, Clone)]
pub struct DirbruteArgs {
    /// Target URL (e.g. https://example.com).
    #[arg(required = true)]
    pub url: String,

    /// Path to the wordlist file (optional).
    #[arg(short = 'w', long = "wordlist")]
    pub wordlist: Option<String>,

    /// Number of concurrent requests to execute (default: 10).
    #[arg(short = 't', long = "threads", default_value_t = 10)]
    pub threads: usize,

    /// Custom HTTP headers (example: "Authorization: Bearer <token>").
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    /// Custom session cookie (example: "session_id=abc-123").
    #[arg(short = 'c', long = "cookie")]
    pub cookie: Option<String>,

    /// Enable Smart Wordlist (built-in patterns for common technologies).
    #[arg(long = "auto-wordlist")]
    pub auto_wordlist: bool,

    /// Enable Adaptive Threading (auto-scaling based on server performance).
    #[arg(long = "auto-threads")]
    pub auto_threads: bool,

    /// Maximum recursion depth (0 = infinite).
    #[arg(short = 'd', long = "depth", default_value_t = 1)]
    pub depth: usize,

    /// Enable real-time logging in the terminal.
    #[arg(short = 'l', long = "show-logs")]
    pub show_logs: bool,

    /// Enable HTML Crawler for dynamic link discovery.
    #[arg(short = 'C', long = "crawler")]
    pub crawler: bool,

    /// Result output file path (e.g. reports/scan.json).
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,

    /// Output format (json or csv).
    #[arg(short = 'f', long = "format")]
    pub format: Option<String>,

    /// Enable Deep Stealth Mode (WAF evasion, decoys, cool-downs).
    #[arg(short = 's', long = "stealth")]
    pub stealth: bool,

    /// Proxy server URL or comma-separated pool (e.g. http://proxy:8080).
    #[arg(short = 'p', long = "proxy")]
    pub proxy: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirbrute_args_parsing() {
        use clap::Parser;
        let args = Cli::parse_from([
            "dirbrute",
            "pentest",
            "dirbrute",
            "https://example.com",
            "--threads",
            "20",
        ]);
        if let Command::Pentest(pentest) = args.command {
            let PentestCommand::Dirbrute(dirbrute) = pentest.pentest_command;
            assert_eq!(dirbrute.url, "https://example.com");
            assert_eq!(dirbrute.threads, 20);
            assert!(!dirbrute.stealth);
        } else {
            panic!("Expected Pentest command");
        }
    }
}
