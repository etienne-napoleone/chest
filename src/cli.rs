use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "chest")]
#[command(about = "A file encryption cli tool")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Create a new chest
    #[command(arg_required_else_help = true)]
    Create { source: String, password: String },
    /// Peek into a chest and list its content
    #[command(arg_required_else_help = true)]
    Peek { path: String },
    /// Open a chest and extract its encrypted content
    #[command(arg_required_else_help = true)]
    Open { path: String },
}
