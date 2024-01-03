use clap::Parser;
use compression::DeflateCompressor;

mod cli;
mod compression;
mod crypto;
mod file;

fn main() {
    let cmd = cli::Cli::parse();

    match cmd.command {
        cli::Commands::Create { source } => println!("create from {source:?}"),
        cli::Commands::Open { path } => {
            let file = file::File::new(path.into());
            let compressor = DeflateCompressor;
            let compressed = file.compress(&compressor).unwrap();
            dbg!(compressed.clone());
            dbg!(compressed.len());
        }
    };
}
