use std::error::Error;
use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

use sum_modpack::Modpack;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let modpack = match Modpack::from_path(&cli.modpack) {
        Ok(modpack) => modpack,
        Err(e) => {
            let filename = cli.modpack.display();
            return Err(format!("Failed to read modpack file '{filename}': {e}").into());
        }
    };

    println!("{modpack:?}");
    if modpack.workshop_ids.len() < 1 {
        return Err("No workshop IDs found".into());
    }

    let stats = modpack.fetch_stats()?;
    println!("{stats:?}");

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>,

    /// The modpack file to read
    modpack: PathBuf,
}
