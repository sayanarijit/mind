use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    name: String,
    start: DateTime<Local>,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            name,
            start: Local::now(),
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn start(&self) -> &DateTime<Local> {
        &self.start
    }
}
