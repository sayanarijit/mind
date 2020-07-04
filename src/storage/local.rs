use crate::{Mind, Reminder, Storage, REMINDER_EXAMPLES};
use dirs;
use serde_yaml;
use std::fs;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};

pub struct LocalStorage {
    mind_path: PathBuf,
}

impl Storage for LocalStorage {
    fn init() -> io::Result<Self> {
        // Check if reminder template is OK
        Reminder::examples();

        let local_storage = dirs::home_dir()
            .expect("failed go get home directory")
            .join(Path::new(".mind"));
        if !local_storage.exists() {
            fs::create_dir(&local_storage)?;
        };

        let mind_file_path = local_storage.join("mind.yml");
        if !mind_file_path.exists() {
            let file = File::create(&mind_file_path)?;
            serde_yaml::to_writer(&file, &Mind::default()).expect("failed to create mind.yml");
        };

        let reminder_examples_path = local_storage.join("reminder_examples.yml");
        let mut file = File::create(&reminder_examples_path)?;
        file.write_all(REMINDER_EXAMPLES.as_bytes())?;

        Ok(Self {
            mind_path: mind_file_path,
        })
    }

    fn load(&self) -> io::Result<Mind> {
        let mind: Mind = serde_yaml::from_reader(BufReader::new(&File::open(&self.mind_path)?))
            .expect("invalid format");
        return Ok(mind);
    }
    fn save(&self, mind: Mind) -> io::Result<()> {
        serde_yaml::to_writer(File::create(&self.mind_path)?, &mind).expect("failed to save file.");
        Ok(())
    }
}
