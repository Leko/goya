use std::fs::create_dir_all;
use std::io;
use std::path::{Path, PathBuf};

pub struct PathUtil {
    base: String,
}
impl PathUtil {
    pub fn from(base: String) -> PathUtil {
        PathUtil { base }
    }

    pub fn mkdirp(&self) -> io::Result<()> {
        create_dir_all(&self.base)
    }

    pub fn da_path(&self) -> PathBuf {
        Path::new(&self.base).join("da.bin")
    }

    pub fn dict_path(&self) -> PathBuf {
        Path::new(&self.base).join("dict.bin")
    }

    pub fn features_path(&self) -> PathBuf {
        Path::new(&self.base).join("features.bin")
    }

    pub fn da_json_path(&self) -> PathBuf {
        Path::new(&self.base).join("da.json")
    }

    pub fn dict_json_path(&self) -> PathBuf {
        Path::new(&self.base).join("dict.json")
    }

    pub fn features_json_path(&self) -> PathBuf {
        Path::new(&self.base).join("features.json")
    }
}
