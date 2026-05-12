use std::fmt::Display;
use std::num::ParseIntError;

use itertools::izip;
use log::warn;
use tabled::settings::object::{Columns, Object};
use tabled::settings::{Alignment, Style};
use tabled::{Table, Tabled};

use crate::payloads;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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

    pub fn to_table(&self) -> Table {
        let mut mods: Vec<Mod> = self.mods.to_vec();
        mods.sort_by(|a, b| a.title.cmp(&b.title));
        mods.sort_by_key(|b| std::cmp::Reverse(b.file_size));

        let mut rows: Vec<ModStatRow> = vec![];

        let sizes_down: Vec<u64> = mods.iter().map(|m| m.file_size).collect();
        let mut sizes_up = sizes_down.clone();
        sizes_up.reverse();

        let total_down = cumulative_sum(&sizes_down);
        let mut total_up = cumulative_sum(&sizes_up);
        total_up.reverse();

        for ((i, m), total_up, total_down) in izip!(mods.iter().enumerate(), total_up, total_down) {
            rows.push(ModStatRow {
                index: i + 1,
                total_up,
                total_down,
                size: m.file_size,
                title: m.title.clone(),
            })
        }

        let mut table = Table::new(rows);
        table.with(Style::rounded());
        table.modify(Columns::new(..).not(Columns::last()), Alignment::right());
        table
    }
}

impl Display for ModpackStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_table().fmt(f)
    }
}

fn cumulative_sum<T: Default + Copy + std::ops::AddAssign>(values: &[T]) -> Vec<T> {
    let mut sums = vec![];
    let mut total = T::default();
    for v in values {
        total += *v;
        sums.push(total);
    }
    sums
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Tabled)]
struct ModStatRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Total (up)", display = "display_filesize")]
    total_up: u64,
    #[tabled(rename = "Total (down)", display = "display_filesize")]
    total_down: u64,
    #[tabled(rename = "Size", display = "display_filesize")]
    size: u64,
    #[tabled(rename = "Title")]
    title: String,
}

fn display_filesize(n: &u64) -> String {
    if *n < 1_000 {
        format!("{n}B")
    } else if *n < 1_000_000 {
        let n = n.div_ceil(1_000);
        format!("{n}KB")
    } else {
        let n = n.div_ceil(1_000_000);

        let mut buf = num_format::Buffer::default();
        buf.write_formatted(&n, &num_format::Locale::en);
        let s = buf.as_str();

        format!("{s}MB")
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
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
