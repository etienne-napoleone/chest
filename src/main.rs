use std::{ffi::OsStr, path::Path};

use chest::{LockedChest, UnlockedChest};
use clap::Parser;
use error::ChestResult;
use term::{fatal, info, prompt, remove_last_lines, success, INFO};

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
            let mut unlocked = UnlockedChest::new(&password, !no_compression);
            success("Created new chest");
            add.iter().try_for_each::<_, ChestResult<()>>(|path| {
                info(&format!("Adding file {}", INFO.apply_to(format_path(path))));
                unlocked.add_file_from_path(path)?;
                remove_last_lines(1);
                success(&format!("Added file {}", INFO.apply_to(format_path(path))));
                Ok(())
            })?;
            let locked = unlocked.lock(&password)?;
            success("Locked chest");
            let path = format!("./{name}.chest");
            info(&format!("Writing chest to {}", INFO.apply_to(&path)));
            locked.write_to_file(&path)?;
            remove_last_lines(1);
            success(&format!("Wrote chest to {}", INFO.apply_to(&path)));
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
            success(&format!(
                "Opened chest {}",
                INFO.apply_to(format_path(&chest))
            ));
            let unlocked = locked.unlock(&password)?;
            success("Unlocked chest");
            let out = out.unwrap_or_else(|| {
                chest
                    .as_path()
                    .file_stem()
                    .unwrap_or(OsStr::new("out"))
                    .into()
            });
            info(&format!(
                "Extracting chest to folder {}",
                INFO.apply_to(format_path(&out))
            ));
            unlocked.decrypt_files_to_folder(&out)?;

            success(&format!(
                "Extracted chest to folder {}",
                INFO.apply_to(format_path(&out))
            ));
        }
    };
    Ok(())
}

fn format_path(path: &Path) -> String {
    let path_string = path.to_str().unwrap_or_default().to_string();
    if path.is_absolute() {
        return path_string;
    }

    format!("./{path_string}")
}
