use std::path::PathBuf;

use clap::Parser;

use sum_modpack::Modpack;

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let modpack = match Modpack::from_path(&cli.modpack) {
        Ok(modpack) => modpack,
        Err(e) => {
            let filename = cli.modpack.display();
            return Err(format!("Failed to read modpack file '{filename}': {e}"))
        }
    };
    println!("{modpack:?}");

    let stats = modpack.fetch_stats();
    println!("{stats:?}");

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The modpack file to read
    modpack: PathBuf,
}
