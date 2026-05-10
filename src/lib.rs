use std::path::PathBuf;

#[derive(Debug)]
pub struct Modpack {}

impl Modpack {
    pub fn from_path(path: &PathBuf) -> Self {
        todo!()
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
