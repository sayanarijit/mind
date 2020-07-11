use crate::{Command, Reminder, Task};
use chrono::Local;
use chrono_humanize::HumanTime;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use std::iter;
use std::process;
use termion::color;
use termion::terminal_size;

// Access it using Mind::version()
static VERSION: &str = "0.3.3";

/// The productive mind.
#[derive(Default)]
pub struct Mind {
    tasks: Vec<Task>,
    reminders: Vec<Reminder>,
}

impl Mind {
    pub fn from(tasks: Vec<Task>, reminders: Vec<Reminder>) -> Self {
        Self { tasks, reminders }
    }

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

    /// Get the version. See ~/.mind/version
    pub fn version() -> &'static str {
        VERSION
    }

    /// Get the pending tasks. See ~/.mind/tasks.yml
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Get the reminders. See ~/.mind/reminders.yml
    pub fn reminders(&self) -> &Vec<Reminder> {
        &self.reminders
    }

    /// Go through the reminders and taks proper action.
    pub fn remind_tasks(&mut self) {
        let now = Local::now();
        let mut new_reminders: Vec<Reminder> = Vec::new();

        for reminder in self.reminders.clone() {
            if reminder.when() > &now {
                new_reminders.push(reminder);
                continue;
            }

            self.push(format!("[reminder] {}", &reminder.name().clone()));
            if let Some(next) = reminder.next() {
                let mut next = next;
                while next.when().clone() <= now {
                    next = next.next().unwrap();
                }
                new_reminders.push(next);
            }
        }
        self.reminders = new_reminders;
    }

    fn edit(&mut self, index: usize) -> io::Result<()> {
        let task = self.tasks.get_mut(index).expect("invalid index");
        let h1 = iter::repeat('=')
            .take(task.name().chars().count())
            .collect::<String>();
        let path = env::temp_dir().join("___mind___tmp_task___.md");

        {
            let mut file = File::create(&path)?;

            write!(
                file,
                "{}\n{}\n\n{}",
                task.name(),
                h1,
                task.details()
                    .clone()
                    .unwrap_or("Write details here...".into())
            )?;
        }

        process::Command::new(env::var("EDITOR").unwrap_or("vi".into()))
            .arg(&path)
            .status()
            .expect("failed to open editor");

        let mut contents = String::new();
        File::open(&path)?.read_to_string(&mut contents)?;
        let mut lines = contents.lines();
        let name = lines.next().expect("missing the task name");
        lines.next();

        let details = lines.collect::<String>();
        let details = details.trim();

        task.edit(
            name.into(),
            if details.chars().count() > 0 {
                Some(details.into())
            } else {
                None
            },
        );

        Ok(())
    }

    /// Act based on the given command.
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
                if index < self.tasks.len() {
                    self.tasks.remove(index);
                }
            }

            Command::PopLast => {
                self.pop();
            }

            Command::Edit(index) => {
                if index < self.tasks.len() {
                    self.edit(index).expect("failed to edit");
                }
            }

            Command::EditLast => {
                if self.tasks.len() > 0 {
                    self.edit(self.tasks.len() - 1).expect("failed to edit");
                }
            }
        }
    }
}

impl fmt::Display for Mind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut color = 155 as u8;
        let len = self.tasks.len();
        let max_name_width = terminal_size().expect("failed to get terminal size").0 as usize - 30;

        let width = self
            .tasks
            .iter()
            .map(|t| t.name().chars().count().min(max_name_width))
            .max()
            .unwrap_or(0);
        let now = Local::now();

        for (task, idx) in self.tasks.iter().zip(0..) {
            let name = task.name().chars().take(max_name_width);
            writeln!(
                f,
                "[{}] {}{:width$}{}\t{}{}{}",
                idx,
                color::Fg(color::Rgb(color - 70, color - 30, color)),
                name.collect::<String>(),
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
