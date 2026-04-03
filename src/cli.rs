use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "secops")]
#[command(about = "ISU SecOps Engine CLI", version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Pentest(PentestArgs),
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
    #[arg(short = 'w', long = "wordlist", required = true)]
    pub wordlist: String,

    /// Number of concurrent threads/requests
    #[arg(short = 't', long = "threads", default_value_t = 10)]
    pub threads: usize,

    /// Custom headers, e.g. "Authorization: Bearer token"
    #[arg(short = 'H', long = "header")]
    pub headers: Vec<String>,

    /// Custom cookies, e.g. "session_id=12345"
    #[arg(short = 'c', long = "cookie")]
    pub cookie: Option<String>,
}
