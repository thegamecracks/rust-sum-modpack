use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PublishedFileDetailsRequest {
    pub itemcount: usize,
    #[serde(flatten)]
    pub publishedfileids: HashMap<String, u64>,
}

impl PublishedFileDetailsRequest {
    pub fn new(workshop_ids: &[u64]) -> Self {
        Self {
            itemcount: workshop_ids.len(),
            publishedfileids: create_publishedfileids(workshop_ids),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PublishedFileDetailsResponse {
    pub response: PublishedFileDetailsResponseResponse,
}

#[derive(Debug, Deserialize)]
pub struct PublishedFileDetailsResponseResponse {
    // FIXME: is there a better name/approach?
    pub publishedfiledetails: Vec<FileDetails>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum FileDetails {
    Ok {
        publishedfileid: u64,
        title: String,
        description: String,
        file_size: u64,
        tags: Vec<Tag>,
    },
    Err {
        publishedfileid: u64,
        result: u64,
    },
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub tag: String,
}

fn create_publishedfileids(workshop_ids: &[u64]) -> HashMap<String, u64> {
    workshop_ids
        .iter()
        .enumerate()
        .map(|(i, id)| (format!("publishedfileids[{i}]"), id.to_owned())) // FIXME: is copy appropriate?
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn parse_payloads_ok() {
        let mut path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        path.push("tests/payloads_ok.json");
        let content = std::fs::read_to_string(path).unwrap();
        serde_json::from_str::<PublishedFileDetailsResponse>(&content).unwrap();
    }

    #[test]
    fn parse_payloads_err() {
        let mut path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        path.push("tests/payloads_err.json");
        let content = std::fs::read_to_string(path).unwrap();
        serde_json::from_str::<PublishedFileDetailsResponse>(&content).unwrap();
    }
}
