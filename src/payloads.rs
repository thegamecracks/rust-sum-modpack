use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PublishedFileDetails {
    #[serde(skip_deserializing)]
    Request {
        itemcount: usize,
        #[serde(flatten)]
        publishedfileids: HashMap<String, u64>,
    },

    #[serde(skip_serializing)]
    Response {
        publishedfiledetails: Vec<FileDetails>,
    },
}

impl PublishedFileDetails {
    pub fn new_request(workshop_ids: &[u64]) -> Self {
        Self::Request {
            itemcount: workshop_ids.len(),
            publishedfileids: create_publishedfileids(workshop_ids),
        }
    }
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
