use crate::Reminder;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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
        &self.details
    }

    pub fn start(&self) -> &DateTime<Local> {
        &self.start
    }

    pub fn edit(&mut self, name: String, details: Option<String>) {
        self.name = name;
        self.details = details;
    }

    pub fn from_reminder(reminder: &Reminder) -> Self {
        let mut task = Self::new(format!("📆 {}", &reminder.name().clone()));
        if let Some(details) = reminder.details() {
            task.details = Some(details.clone());
        }
        task
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hr = iter::repeat("=")
            .take(self.name.chars().count())
            .collect::<String>();
        let details = self
            .details
            .clone()
            .unwrap_or_else(|| "No details...".into());

        writeln!(f, "{}", &self.name)?;
        writeln!(f, "{}", hr)?;
        write!(f, "{}", details)
    }
}
