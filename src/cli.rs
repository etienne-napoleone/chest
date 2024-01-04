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
    New { name: String, password: String },
    /// Create a new chest
    #[command(arg_required_else_help = true)]
    AddFile {
        chest_path: String,
        file_path: String,
        password: String,
    },
    /// Peek into a chest and list its content
    #[command(arg_required_else_help = true)]
    Peek {
        chest_path: String,
        password: String,
    },
    /// Open a chest and extract its encrypted content
    #[command(arg_required_else_help = true)]
    Open { chest_path: String },
}
