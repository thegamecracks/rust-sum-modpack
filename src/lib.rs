pub mod payloads;
pub mod stats;

use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Modpack {
    pub workshop_ids: Vec<u64>,
}

impl Modpack {
    const STEAMAPI_FILEDETAILS_URL: &str =
        "https://api.steampowered.com/ISteamRemoteStorage/GetPublishedFileDetails/v1/";

    pub fn from_path(path: &Path) -> Result<Self, ModpackError> {
        let content = read_to_string(path)?;
        Ok(Self::from_str(&content)?)
    }

    pub fn fetch_stats(&self) -> Result<stats::ModpackStats, Box<dyn Error>> {
        let request = payloads::PublishedFileDetailsRequest::new(&self.workshop_ids);
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(Self::STEAMAPI_FILEDETAILS_URL)
            .form(&request)
            .send()?;

        // let mut content = String::new();
        // response.read_to_string(&mut content)?;
        // println!("{content}");

        let response = response.json::<payloads::PublishedFileDetailsResponse>()?;
        Ok(stats::ModpackStats::from_response(&response)?)
    }
}

impl FromStr for Modpack {
    type Err = ();

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"https://steamcommunity.com/sharedfiles/filedetails/\?id=(\d+)")
                .expect("Workshop ID pattern is not valid")
        });

        let mut workshop_ids: Vec<u64> = PATTERN
            .captures_iter(content)
            .map(|c| {
                c.get(1)
                    .expect("Missing capture group")
                    .as_str()
                    .parse()
                    .expect("Capture group does not contain a valid u64")
            })
            .collect();

        let mut unique_workshop_ids = std::collections::HashSet::new();
        workshop_ids.retain(|id| unique_workshop_ids.insert(*id));

        Ok(Self { workshop_ids })
    }
}

pub enum ModpackError {
    IOError(std::io::Error),
    Unknown,
}

impl From<std::io::Error> for ModpackError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<()> for ModpackError {
    fn from(_value: ()) -> Self {
        Self::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deduplicate_workshop_ids() {
        let modpack = Modpack::from_str(
            r#"
            https://steamcommunity.com/sharedfiles/filedetails/?id=123
            https://steamcommunity.com/sharedfiles/filedetails/?id=123
            "#,
        )
        .unwrap();
        assert_eq!(
            modpack.workshop_ids,
            vec![123],
            "workshop IDs must be de-duplicated",
        );
    }
}
