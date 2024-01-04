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
        cli::Commands::New { name, password } => {
            let chest = Chest::new(&password).unwrap();
            let locked = chest.lock(&password).unwrap();
            locked.write_to_file(format!("{name}.chest")).unwrap();
        }
        cli::Commands::Peek { path } => {
            todo!()
        }
        cli::Commands::Open { path } => println!("open {path}"),
    };
}
