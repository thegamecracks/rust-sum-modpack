use std::path::PathBuf;

use clap::Parser;

use sum_modpack::Modpack;

fn main() {
    let cli = Cli::parse();

    let modpack = Modpack::from_path(&cli.modpack);
    println!("{modpack:?}");

    let stats = modpack.fetch_stats();
    println!("{stats:?}");
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The modpack file to read
    modpack: PathBuf,
}
