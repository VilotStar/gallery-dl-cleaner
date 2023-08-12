use clap::Parser;
use std::path::PathBuf;

#[derive(clap::Subcommand)]
pub enum HandlerCommand {
    E621,
    DotParty
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// gallery-dl path to clean
    #[arg(short, long, value_name = "PATH")]
    pub path: PathBuf,
    /// provider of given path
    #[command(subcommand)]
    pub subcommand: HandlerCommand
}
