use std::ffi::OsStr;

use chest::{LockedChest, UnlockedChest};
use clap::Parser;
use error::ChestResult;
use term::{fatal, prompt};

mod chest;
mod cli;
mod compression;
mod crypto;
mod error;
mod key;
mod random;
mod term;

fn main() {
    if let Err(e) = run() {
        fatal(&e.to_string(), 1);
    }
}

fn run() -> ChestResult<()> {
    let cmd = cli::Cli::parse();
    match cmd.command {
        cli::Commands::New {
            name,
            password,
            add,
            no_compression,
        } => {
            let password = password.unwrap_or_else(|| prompt("Password"));
            let mut unlocked = UnlockedChest::new(&password, !no_compression)?;
            add.iter()
                .try_for_each(|path| unlocked.add_file_from_path(path))?;
            let locked = unlocked.lock(&password)?;
            locked.write_to_file(format!("{name}.chest"))?;
        }
        cli::Commands::Peek { chest, password } => {
            let password = password.unwrap_or_else(|| prompt("Password"));
            let locked = LockedChest::from_file(chest)?;
            let unlocked = locked.unlock(&password)?;
            unlocked
                .files
                .iter()
                .for_each(|f| println!("{}", f.metadata.filename));
        }
        cli::Commands::Open {
            chest,
            out,
            password,
        } => {
            let password = password.unwrap_or_else(|| prompt("Password"));
            let locked = LockedChest::from_file(&chest)?;
            let unlocked = locked.unlock(&password)?;
            let out = out.unwrap_or_else(|| {
                chest
                    .as_path()
                    .file_stem()
                    .unwrap_or(OsStr::new("out"))
                    .into()
            });
            unlocked.decrypt_files_to_folder(out)?;
        }
    };
    Ok(())
}
