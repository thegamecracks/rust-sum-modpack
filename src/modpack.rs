use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;
use thiserror::Error;

use crate::payloads;
use crate::stats;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Modpack {
    pub workshop_ids: Vec<u64>,
}

impl Modpack {
    const STEAMAPI_FILEDETAILS_URL: &str =
        "https://api.steampowered.com/ISteamRemoteStorage/GetPublishedFileDetails/v1/";

    pub fn from_path(path: &Path) -> Result<Self, std::io::Error> {
        let content = read_to_string(path)?;
        Ok(Self::from_str(&content).expect("from_str() should not fail"))
    }

    pub fn fetch_stats(&self) -> Result<stats::ModpackStats, FetchStatsError> {
        let request = payloads::PublishedFileDetailsRequest::new(&self.workshop_ids);
        let response = ureq::post(Self::STEAMAPI_FILEDETAILS_URL)
            .content_type("application/x-www-form-urlencoded")
            .send(serde_urlencoded::ser::to_string(request)?)?
            .body_mut()
            .read_json::<payloads::PublishedFileDetailsResponse>()?;
        // TODO: consider logging body here
        Ok(stats::ModpackStats::from_response(&response))
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

#[derive(Debug, Error)]
pub enum FetchStatsError {
    #[error("{0}")]
    FormSerializationError(#[from] serde_urlencoded::ser::Error),
    #[error("{0}")]
    RequestError(#[from] ureq::Error),
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
