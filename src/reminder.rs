use chrono::{DateTime, Local, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum Repeat {
    Never,
    EveryDay,
    EveryWeek,
    Weekly(Vec<Weekday>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Reminder {
    name: String,
    when: DateTime<Local>,
    repeat: Repeat,
}

impl Reminder {
    pub fn new(name: String, when: DateTime<Local>, repeat: Repeat) -> Self {
        Self { name, when, repeat }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn when(&self) -> &DateTime<Local> {
        &self.when
    }
    pub fn repeat(&self) -> &Repeat {
        &self.repeat
    }
}
