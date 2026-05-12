use std::error::Error;
use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use tracing::{debug, info};

use sum_modpack::modpack::{Modpack, ModpackError};
use sum_modpack::stats::SortMode;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    pretty_env_logger::formatted_builder()
        .filter_level(cli.verbosity.log_level_filter())
        .init();

    let modpack = match Modpack::from_path(&cli.modpack) {
        Ok(modpack) => modpack,
        Err(e) => match e {
            ModpackError::IOError(e) => {
                let filename = cli.modpack.display();
                return Err(format!("Failed to read modpack file '{filename}': {e}").into());
            }
            ModpackError::Unknown => {
                return Err("Unknown error".into());
            }
        },
    };

    debug!("{modpack:?}");
    let len = modpack.workshop_ids.len();
    if len < 1 {
        return Err("No workshop IDs found".into());
    }

    info!("Fetching {len} workshop mods");
    let stats = modpack.fetch_stats()?;
    debug!("{stats:?}");

    let sort = cli.sort.unwrap_or(SortMode::default());
    let table = stats.to_table(sort);
    println!("{table}");

    Ok(())
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// How mods are sorted in the table
    #[arg(short, long, value_enum)]
    sort: Option<SortMode>,

    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>,

    /// The modpack file to read
    modpack: PathBuf,
}
