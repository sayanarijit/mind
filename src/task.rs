use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    start: DateTime<Utc>,
    name: String,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            name,
            start: Utc::now(),
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn start(&self) -> &DateTime<Utc> {
        &self.start
    }
}
