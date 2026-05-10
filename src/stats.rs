use std::num::ParseIntError;

use log::warn;

use crate::payloads;

#[derive(Debug)]
pub struct ModpackStats {
    pub mods: Vec<Mod>,
}

impl ModpackStats {
    pub fn from_response(
        response: &payloads::PublishedFileDetailsResponse,
    ) -> Result<Self, String> {
        let mut mods = vec![];
        for details in response.response.publishedfiledetails.iter() {
            match Mod::try_from(details) {
                Ok(m) => mods.push(m),
                Err(e) => match e {
                    ModError::ItemNotFound(publishedfileid) => {
                        warn!("Item ID not found: {}", publishedfileid)
                    }
                    ModError::InvalidResult(publishedfileid, result) => warn!(
                        "Item ID {} Unexpected result code: {}",
                        publishedfileid, result,
                    ),
                    ModError::ParseIntError(e) => warn!("{}", e),
                },
            }
        }
        Ok(Self { mods })
    }
}

#[derive(Debug)]
pub struct Mod {
    pub publishedfileid: u64,
    pub title: String,
    pub file_size: u64,
}

impl TryFrom<&payloads::FileDetails> for Mod {
    type Error = ModError;

    fn try_from(value: &payloads::FileDetails) -> Result<Self, Self::Error> {
        match value {
            payloads::FileDetails::Ok {
                publishedfileid,
                title,
                file_size,
                tags: _,
            } => {
                let publishedfileid = publishedfileid.parse()?;
                // .map_err(|e| format!("Item ID is invalid: {publishedfileid}"))?;

                let file_size = file_size.parse()?;
                // .map_err(|e| format!("File size is invalid: {file_size}"))?;

                Ok(Self {
                    publishedfileid,
                    title: title.to_string(),
                    file_size,
                })
            }
            payloads::FileDetails::Err {
                publishedfileid,
                result,
            } => {
                if *result == 9 {
                    Err(ModError::ItemNotFound(publishedfileid.to_owned()))
                } else {
                    Err(ModError::InvalidResult(publishedfileid.to_owned(), *result))
                }
            }
        }
    }
}

pub enum ModError {
    ItemNotFound(String),
    InvalidResult(String, u64),
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for ModError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
