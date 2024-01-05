use std::io::{self, Write};

use anyhow::Result;
use chest::{LockedChest, UnlockedChest};
use clap::Parser;

mod chest;
mod cli;
mod compression;
mod crypto;
mod key;
mod random;

fn main() -> Result<()> {
    let cmd = cli::Cli::parse();
    match cmd.command {
        cli::Commands::New {
            name,
            password,
            add,
        } => {
            let password = prompt_password_if_empty(password);
            let mut unlocked = UnlockedChest::new(&password)?;
            add.iter()
                .try_for_each(|path| unlocked.add_file_from_path(path))?;
            let locked = unlocked.lock(&password)?;
            locked.write_to_file(format!("{name}.chest"))?;
        }
        cli::Commands::Peek { chest, password } => {
            let password = prompt_password_if_empty(password);
            let locked = LockedChest::from_file(chest)?;
            let unlocked = locked.unlock(&password)?;
            unlocked
                .files
                .iter()
                .for_each(|f| println!("{}", f.metadata.filename));
        }
        cli::Commands::Open { .. } => println!("open"),
    };
    Ok(())
}

fn prompt_password_if_empty(password: Option<String>) -> String {
    password.unwrap_or_else(|| {
        print!("> password: ");
        _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("unable to read user input");
        input.trim().to_string()
    })
}
