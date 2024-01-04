use chest::Chest;
use clap::Parser;

mod chest;
mod cli;
mod compression;
mod crypto;
mod key;
mod random;

fn main() {
    let cmd = cli::Cli::parse();
    match cmd.command {
        cli::Commands::New { name } => {
            let chest = Chest::default();
            chest.write_to_file(format!("{name}.chest")).unwrap();
        }
        cli::Commands::Peek { path } => {
            let chest = Chest::open_from_file(path).unwrap();
            chest
                .files
                .iter()
                .for_each(|f| println!("{} - {}B", f.metadata.filename, f.metadata.size_bytes));
        }
        cli::Commands::Open { path } => println!("open {path}"),
    };
}
