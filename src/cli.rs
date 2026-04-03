use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "secops")]
#[command(about = "ISU SecOps Engine CLI", version = "0.4.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Pentest(PentestArgs),
    Web(WebArgs),
}

#[derive(Parser, Debug)]
pub struct WebArgs {
    /// Port for the web interface
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

#[derive(Parser, Debug)]
pub struct PentestArgs {
    #[command(subcommand)]
    pub pentest_command: PentestCommand,
}

#[derive(Subcommand, Debug)]
pub enum PentestCommand {
    Dirbrute(DirbruteArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct DirbruteArgs {
    /// Target URL
    #[arg(required = true)]
    pub url: String,

    /// Path to the wordlist file
    #[arg(short = 'w', long = "wordlist")]
    pub wordlist: Option<String>,

    /// Number of concurrent threads/requests
    #[arg(short = 't', long = "threads", default_value_t = 10)]
    pub threads: usize,

    /// Custom headers, e.g. "Authorization: Bearer token"
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    /// Custom cookies, e.g. "session_id=12345"
    #[arg(short = 'c', long = "cookie")]
    pub cookie: Option<String>,

    /// Enable Smart Wordlist (Static Patterning)
    #[arg(long = "auto-wordlist")]
    pub auto_wordlist: bool,

    /// Enable Adaptive Threading (Auto performance scaling)
    #[arg(long = "auto-threads")]
    pub auto_threads: bool,

    /// Show real-time scan logs in terminal
    #[arg(short = 'l', long = "show-logs")]
    pub show_logs: bool,
}
