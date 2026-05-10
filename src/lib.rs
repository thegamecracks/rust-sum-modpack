use std::{fs::read_to_string, path::Path, sync::LazyLock};

use regex::Regex;

#[derive(Debug)]
pub struct Modpack {
    pub workshop_ids: Vec<u64>,
}

impl Modpack {
    pub fn from_path(path: &Path) -> Result<Self, std::io::Error> {
        let content = read_to_string(path)?;
        Ok(Self::from_str(&content))
    }

    pub fn from_str(content: &str) -> Self {
        static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"https://steamcommunity.com/sharedfiles/filedetails/\?id=(\d+)")
                .expect("Workshop ID pattern is not valid")
        });

        let workshop_ids = PATTERN
            .captures_iter(content)
            .map(|c| {
                c.get(1)
                    .expect("Missing capture group")
                    .as_str()
                    .parse()
                    .expect("Capture group does not contain a valid u64")
            })
            .collect();

        Self { workshop_ids }
    }

    pub fn fetch_stats(&self) -> ModpackStats {
        todo!()
    }
}

#[derive(Debug)]
pub struct ModpackStats {}

impl ModpackStats {
    pub fn fetch_from_modpack(modpack: &Modpack) -> Self {
        todo!()
    }
}
