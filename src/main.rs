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
        cli::Commands::AddFile {
            chest_path,
            file_path,
            password,
        } => {
            let locked = Chest::from_file(&chest_path).unwrap();
            let mut chest = locked.unlock(&password).unwrap();
            chest.add_file_from_path(&file_path).unwrap();
            let locked = chest.lock(&password).unwrap();
            locked.write_to_file(&chest_path).unwrap();
        }
        cli::Commands::Peek {
            chest_path,
            password,
        } => {
            let locked = Chest::from_file(chest_path).unwrap();
            let chest = locked.unlock(&password).unwrap();
            chest
                .files
                .iter()
                .for_each(|f| println!("{}", f.metadata.filename));
        }
        cli::Commands::Open { chest_path } => println!("open {chest_path}"),
    };
}
