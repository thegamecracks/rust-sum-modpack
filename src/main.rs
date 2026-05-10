use std::error::Error;
use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use log::{debug, info};

use sum_modpack::Modpack;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(cli.verbosity.log_level_filter())
        .init();

    let modpack = match Modpack::from_path(&cli.modpack) {
        Ok(modpack) => modpack,
        Err(e) => {
            let filename = cli.modpack.display();
            return Err(format!("Failed to read modpack file '{filename}': {e}").into());
        }
    };

    debug!("{modpack:?}");
    let len = modpack.workshop_ids.len();
    if len < 1 {
        return Err("No workshop IDs found".into());
    }

    info!("Fetching {len} workshop mods");
    let stats = modpack.fetch_stats()?;
    debug!("{stats:?}");

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
