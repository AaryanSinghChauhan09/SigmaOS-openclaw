#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "sigmaos-openclaw", about = "OpenClaw autonomous agent integration for SigmaOS")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("sigmaos-openclaw: no entries configured."),
        Commands::Status => println!("sigmaos-openclaw: ready."),
    }
}
