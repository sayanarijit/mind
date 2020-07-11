use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    name: String,
    details: Option<String>,
    start: DateTime<Local>,
}

impl Task {
    pub fn new(name: String) -> Self {
        Self {
            name,
            details: None,
            start: Local::now(),
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn details(&self) -> &Option<String> {
        return &self.details;
    }

    pub fn start(&self) -> &DateTime<Local> {
        &self.start
    }

    pub fn edit(&mut self, name: String, details: Option<String>) {
        self.name = name;
        self.details = details;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hr = iter::repeat("=")
            .take(self.name.chars().count())
            .collect::<String>();
        let details = self.details.clone().unwrap_or("No details...".into());

        writeln!(f, "{}", &self.name)?;
        writeln!(f, "{}", hr)?;
        write!(f, "{}", details)
    }
}
