use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct PublishedFileDetailsRequest {
    pub itemcount: usize,
    #[serde(flatten)]
    pub publishedfileids: HashMap<String, u64>,
}

impl PublishedFileDetailsRequest {
    pub fn new(workshop_ids: &[u64]) -> Self {
        Self {
            itemcount: workshop_ids.len(),
            publishedfileids: create_publishedfileids(workshop_ids.iter().copied()),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct PublishedFileDetailsResponse {
    pub response: PublishedFileDetailsResponseResponse,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct PublishedFileDetailsResponseResponse {
    // FIXME: is there a better name/approach?
    pub publishedfiledetails: Vec<FileDetails>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum FileDetails {
    Ok {
        publishedfileid: String, // FIXME: can this be deserialized into u64?
        title: String,
        file_size: String, // FIXME: can this be deserialized into u64?
        tags: Vec<Tag>,
    },
    Err {
        publishedfileid: String, // FIXME: can this be deserialized into u64?
        result: u64,
    },
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct Tag {
    pub tag: String,
}

fn create_publishedfileids<T: Iterator<Item = u64>>(workshop_ids: T) -> HashMap<String, u64> {
    workshop_ids
        .enumerate()
        .map(|(i, id)| (format!("publishedfileids[{i}]"), id))
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
        let response = serde_json::from_str::<PublishedFileDetailsResponse>(&content).unwrap();

        for details in response.response.publishedfiledetails {
            match details {
                FileDetails::Ok { .. } => (),
                FileDetails::Err { .. } => panic!("Expected Ok, got {:?}", details),
            };
        }
    }

    #[test]
    fn parse_payloads_err() {
        let mut path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        path.push("tests/payloads_err.json");

        let content = std::fs::read_to_string(path).unwrap();
        let response = serde_json::from_str::<PublishedFileDetailsResponse>(&content).unwrap();

        let [valid, invalid] = &response.response.publishedfiledetails[..] else {
            panic!(
                "Expected 2 elements, got {}",
                response.response.publishedfiledetails.len()
            );
        };

        match valid {
            FileDetails::Ok { .. } => (),
            FileDetails::Err { .. } => panic!("Expected Ok, got {:?}", valid),
        };

        match invalid {
            FileDetails::Ok { .. } => panic!("Expected Err, got {:?}", invalid),
            FileDetails::Err { .. } => (),
        };
    }
}
