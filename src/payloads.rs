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
        .map(|id| (format!("publishedfileids[{id}]"), id.to_owned())) // FIXME: is copy appropriate?
        .collect()
}
