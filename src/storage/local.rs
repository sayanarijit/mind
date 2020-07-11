use crate::{Mind, Reminder, Storage, REMINDER_EXAMPLES};
use dirs;
use serde_yaml;
use std::fs;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

pub struct LocalStorage {
    mind_tasks_path: PathBuf,
    mind_reminders_path: PathBuf,
}

impl Storage for LocalStorage {
    fn init() -> io::Result<Self> {
        // Check if reminder template is OK
        Reminder::examples();

        let local_storage = dirs::home_dir()
            .expect("failed go get home directory")
            .join(".mind");
        if !local_storage.exists() {
            fs::create_dir(&local_storage)?;
        };

        let mind_tasks_path = local_storage.join("tasks.yml");
        let default_mind = Mind::default();
        if !mind_tasks_path.exists() {
            let file = File::create(&mind_tasks_path)?;
            serde_yaml::to_writer(&file, default_mind.tasks()).expect("failed to create tasks.yml");
        };

        let mind_reminders_path = local_storage.join("reminders.yml");
        if !mind_reminders_path.exists() {
            let file = File::create(&mind_reminders_path)?;
            serde_yaml::to_writer(&file, default_mind.tasks())
                .expect("failed to create reminders.yml");
        };

        let reminder_examples_path = local_storage.join("reminder_examples.yml");
        let mut file = File::create(&reminder_examples_path)?;
        file.write_all(REMINDER_EXAMPLES.as_bytes())?;

        // TODO Add version compatibility logic
        let mind_version_path = local_storage.join("version");
        let mut file = File::create(&mind_version_path)?;
        file.write_all(Mind::version().as_bytes())?;

        Ok(Self {
            mind_tasks_path,
            mind_reminders_path,
        })
    }

    fn load(&self) -> io::Result<Mind> {
        let mind: Mind = Mind::from(
            serde_yaml::from_reader(BufReader::new(&File::open(&self.mind_tasks_path)?))
                .expect("invalid format"),
            serde_yaml::from_reader(BufReader::new(&File::open(&self.mind_reminders_path)?))
                .expect("invalid format"),
        );
        return Ok(mind);
    }
    fn save(&self, mind: Mind) -> io::Result<()> {
        serde_yaml::to_writer(File::create(&self.mind_tasks_path)?, mind.tasks())
            .expect("failed to save file.");
        serde_yaml::to_writer(File::create(&self.mind_reminders_path)?, mind.reminders())
            .expect("failed to save file.");
        Ok(())
    }
}
