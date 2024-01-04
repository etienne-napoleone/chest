use chest::Chest;
use clap::Parser;
use compression::DeflateCompressor;
use crypto::Aes256Encryptor;

mod chest;
mod cli;
mod compression;
mod crypto;

fn main() {
    let cmd = cli::Cli::parse();

    match cmd.command {
        cli::Commands::Create { source, password } => {
            let compressor = DeflateCompressor;
            let encryptor = Aes256Encryptor;
            let chest =
                Chest::create_from_file(source, &password, &compressor, &encryptor).unwrap();
            chest.write_to_file("./myfiles.chest").unwrap();
        }
        cli::Commands::Peek { path } => {
            let chest = Chest::open_from_file(path).unwrap();
        }
        cli::Commands::Open { path } => println!("open {path}"),
    };
}
