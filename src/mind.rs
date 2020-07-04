use crate::{Command, Reminder, Task};
use chrono::Local;
use chrono_humanize::HumanTime;
use serde::{Deserialize, Serialize};
use std::fmt;
use termion::color;

#[derive(Default, Serialize, Deserialize)]
pub struct Mind {
    #[serde(default)]
    reminders: Vec<Reminder>,
    #[serde(default)]
    tasks: Vec<Task>,
}

impl Mind {
    fn push(&mut self, name: String) {
        if let Some((_task, idx)) = self
            .tasks
            .iter()
            .zip(0..)
            .filter(|(task, _idx)| task.name().trim() == name.trim())
            .next()
        {
            let task = self.tasks.remove(idx);
            self.tasks.push(task);
        } else {
            self.tasks.push(Task::new(name));
        }
    }
    fn pop(&mut self) -> Option<Task> {
        self.tasks.pop()
    }

    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn remind_tasks(&mut self) {
        let now = Local::now();
        let mut new_reminders: Vec<Reminder> = Vec::new();

        for reminder in self.reminders.clone() {
            if reminder.when() > &now {
                new_reminders.push(reminder);
                continue;
            }
            self.push(reminder.name().clone());

            let mut next = reminder.repeat().when_next(reminder.when().clone());
            if next.is_none() {
                continue;
            };

            while next.unwrap() <= now {
                next = reminder.repeat().when_next(next.unwrap());
            }

            new_reminders.push(Reminder::new(
                reminder.name().clone(),
                next.unwrap(),
                reminder.repeat().clone(),
            ));
        }
        self.reminders = new_reminders;
    }

    pub fn act(&mut self, command: Command) {
        match command {
            Command::Push(name) => {
                self.push(name);
            }
            Command::Continue(index) => {
                let task = self.tasks.remove(index);
                self.tasks.push(task);
            }
            Command::Pop(index) => {
                self.tasks.remove(index);
            }
            Command::PopLast => {
                self.pop();
            }
        }
    }
}

impl fmt::Display for Mind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut color = 155 as u8;
        let len = self.tasks.len();

        let width = self
            .tasks
            .iter()
            .map(|t| t.name().chars().count())
            .max()
            .unwrap_or(0);
        let now = Local::now();

        for (task, idx) in self.tasks.iter().zip(0..) {
            writeln!(
                f,
                "[{}] {}{:width$}{}\t{}{}{}",
                idx,
                color::Fg(color::Rgb(color - 70, color - 30, color)),
                &task.name(),
                color::Fg(color::Reset),
                color::Fg(color::Rgb(color - 50, color - 50, color - 50)),
                &HumanTime::from(*task.start() - now),
                color::Fg(color::Reset),
                width = width
            )?;
            color += 100 as u8 / len as u8;
        }
        Ok(())
    }
}
