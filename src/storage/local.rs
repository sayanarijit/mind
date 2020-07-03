use crate::{Mind, Storage};
use dirs;
use serde_yaml;
use std::fs;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

pub struct LocalStorage {
    path: PathBuf,
}

impl Storage for LocalStorage {
    fn init() -> io::Result<Self> {
        let local_storage = dirs::home_dir()
            .expect("failed go get home directory")
            .join(Path::new(".mind"));
        if !local_storage.exists() {
            fs::create_dir(&local_storage)?;
        };

        let mind_file_path = local_storage.join(Path::new("mind.yml"));
        if !mind_file_path.exists() {
            let file = File::create(&mind_file_path)?;
            serde_yaml::to_writer(&file, &Mind::default()).unwrap();
        };

        Ok(Self {
            path: mind_file_path,
        })
    }

    fn load(&self) -> io::Result<Mind> {
        let mind: Mind = serde_yaml::from_reader(BufReader::new(&File::open(&self.path)?)).expect("invalid format");
        return Ok(mind);
    }
    fn save(&self, mind: Mind) -> io::Result<()> {
        serde_yaml::to_writer(File::create(&self.path)?, &mind).expect("failed to save file.");
        Ok(())
    }
}
