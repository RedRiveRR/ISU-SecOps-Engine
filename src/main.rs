mod cli;
mod scanner;
mod web;

use clap::Parser;
use cli::{Cli, Command, PentestCommand};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Pentest(pentest_args) => {
            match pentest_args.pentest_command {
                PentestCommand::Dirbrute(args) => {
                    scanner::run_dirbrute(args).await;
                }
            }
        }
        Command::Web(web_args) => {
            web::start_server(web_args.port).await;
        }
    }
}
