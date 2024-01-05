use std::path::PathBuf;

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
    New {
        /// Chest name
        name: String,
        /// Files to encrypt in the chest
        #[arg(short, long, required = true, num_args(0..), value_name = "PATH")]
        add: Vec<PathBuf>,
        /// Optional chest password, will be prompted if not provided
        #[clap(short, long)]
        password: Option<String>,
    },

    /// Peek into a chest and list its content, decrypting only metadata
    #[command(arg_required_else_help = true)]
    Peek {
        /// Chest file path
        #[clap(value_name = "PATH")]
        chest: PathBuf,
        /// Optional chest password, will be prompted if not provided
        #[clap(short, long)]
        password: Option<String>,
    },

    /// Open a chest and extract its encrypted content
    #[command(arg_required_else_help = true)]
    Open {
        /// Chest file path
        #[clap(value_name = "PATH")]
        chest: PathBuf,
        /// Chest file path
        #[clap(short, long, value_name = "PATH")]
        out: Option<PathBuf>,
        /// Optional chest password, will be prompted if not provided
        #[clap(short, long)]
        password: Option<String>,
    },
}
