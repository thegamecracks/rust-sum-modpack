mod payloads;

use std::error::Error;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

#[derive(Debug)]
pub struct Modpack {
    pub workshop_ids: Vec<u64>,
}

impl Modpack {
    const STEAMAPI_FILEDETAILS_URL: &str =
        "https://api.steampowered.com/ISteamRemoteStorage/GetPublishedFileDetails/v1/";

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

    pub fn fetch_stats(&self) -> Result<ModpackStats, Box<dyn Error>> {
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
        Ok(ModpackStats::from_response(&response)?)
    }
}

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
            match details {
                payloads::FileDetails::Ok {
                    publishedfileid,
                    title,
                    description,
                    file_size,
                    tags: _,
                } => {
                    mods.push(Mod {
                        publishedfileid: *publishedfileid,
                        title: title.to_string(),
                        description: description.to_string(),
                        file_size: *file_size,
                    });
                }
                payloads::FileDetails::Err {
                    publishedfileid,
                    result,
                } => {
                    return if *result == 9 {
                        Err(format!("Item ID not found: {publishedfileid}"))
                    } else {
                        Err(format!(
                            "Item ID {publishedfileid} Unexpected result code: {result}"
                        ))
                    };
                }
            }
        }
        Ok(Self { mods })
    }
}

#[derive(Debug)]
pub struct Mod {
    pub publishedfileid: u64,
    pub title: String,
    pub description: String,
    pub file_size: u64,
}
