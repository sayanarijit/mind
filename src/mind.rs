use crate::{Command, Productivity, Reminder, Task};
use atty;
use chrono::Duration;
use chrono::Local;
use chrono_humanize::HumanTime;
use serde_yaml;
use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::process;
use termion::color;
use termion::terminal_size;

// Access it using Mind::version()
static VERSION: &str = "0.6.0";

/// The productive mind.
#[derive(Default)]
pub struct Mind {
    tasks: Vec<Task>,
    reminders: Vec<Reminder>,
    focused: Option<usize>,
}

impl Mind {
    pub fn from(tasks: Vec<Task>, reminders: Vec<Reminder>) -> Self {
        Self {
            tasks,
            reminders,
            focused: None,
        }
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

    /// Get the focused task
    pub fn focused(&self) -> Option<&Task> {
        self.focused
            .map(|idx| self.tasks.get(idx).map(|t| Some(t)).unwrap_or(None))
            .unwrap_or(None)
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

            self.push(format!("📆 {}", &reminder.name().clone()));
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

    // Total backlog
    pub fn backlog(&self) -> Duration {
        let now = Local::now();
        self.tasks
            .iter()
            .map(|t| (now - *t.start()))
            .fold(Duration::zero(), |x, y| x + y)
    }

    // Productivity from backlog
    pub fn productivity(&self) -> Productivity {
        Productivity::from_backlog(self.backlog())
    }

    fn edit(&mut self, index: usize) -> io::Result<()> {
        let task = self.tasks.get_mut(index).expect("invalid index");
        let path = env::temp_dir().join("___mind___tmp_task___.md");

        {
            let mut file = fs::File::create(&path)?;
            write!(file, "{}", task)?;
        }

        process::Command::new(env::var("EDITOR").unwrap_or("vi".into()))
            .arg(&path)
            .status()
            .expect("failed to open editor");

        let mut contents = String::new();
        fs::File::open(&path)?.read_to_string(&mut contents)?;
        let mut lines = contents.lines();
        let name = lines.next().expect("missing the task name");
        lines.next();

        let details = lines.collect::<Vec<&str>>().join("\n");
        let details = details.trim();

        task.edit(
            name.into(),
            if details.chars().count() > 0 {
                Some(details.into())
            } else {
                None
            },
        );

        fs::remove_file(path)
    }

    fn edit_reminders(&mut self) -> io::Result<()> {
        let reminders = self.reminders();
        let lines: Vec<String> = serde_yaml::to_string(reminders)
            .expect("failed to encode reminders")
            .lines()
            .map(String::from)
            .chain(["#", "# # Examples"].iter().map(|l| l.to_string()))
            .chain(Reminder::examples().lines().map(|l| format!("# {}", l)))
            .collect();

        let path = env::temp_dir().join("___mind___tmp_reminders___.yml");
        {
            let mut file = fs::File::create(&path)?;
            write!(file, "{}", lines.join("\n"))?;
        }

        process::Command::new(env::var("EDITOR").unwrap_or("vi".into()))
            .arg(&path)
            .status()
            .expect("failed to open editor");

        let mut contents = String::new();
        fs::File::open(&path)?.read_to_string(&mut contents)?;

        self.reminders =
            serde_yaml::from_reader(BufReader::new(&File::open(&path)?)).expect("invalid format");

        fs::remove_file(path)
    }

    /// Act based on the given command.
    pub fn act(&mut self, command: Command) {
        self.focused = None;

        match command {
            Command::Push(name) => {
                self.push(name);
            }

            Command::Continue(index) => {
                if index < self.tasks.len() {
                    let task = self.tasks.remove(index);
                    self.tasks.push(task);
                }
            }

            Command::Get(index) => {
                if index < self.tasks.len() {
                    self.focused = Some(index);
                }
            }

            Command::GetLast => {
                if self.tasks.len() > 0 {
                    self.focused = Some(self.tasks.len() - 1);
                }
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

            Command::EditReminders => self.edit_reminders().expect("failed to edit"),
        }
    }
}

impl fmt::Display for Mind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut color = 155 as u8;
        let len = self.tasks.len();
        let max_name_width = terminal_size().unwrap_or((100, 0)).0 as usize - 30;

        let name_width = self
            .tasks
            .iter()
            .map(|t| t.name().chars().count().min(max_name_width))
            .max()
            .unwrap_or(0);
        let idx_width = len.to_string().chars().count();

        let now = Local::now();

        for (task, idx) in self.tasks.iter().zip(0..) {
            let name = task.name().chars().take(max_name_width);

            let name_color = match self.productivity() {
                Productivity::Optimal => color::Fg(color::Rgb(0, color, 0)),
                Productivity::High => color::Fg(color::Rgb(color - 75, color - 25, 0)),
                Productivity::Normal => color::Fg(color::Rgb(color - 50, color - 50, 0)),
                Productivity::Low => color::Fg(color::Rgb(color - 25, color - 75, 0)),
                Productivity::UnProductive => color::Fg(color::Rgb(color, 0, 0)),
            };

            if atty::is(atty::Stream::Stdout) {
                write!(
                    f,
                    "[{idx:idx_width$}] {name_color}{name:name_width$}\t{age_color}{age}{reset_color}",
                    idx = idx,
                    idx_width = idx_width,
                    name_color = name_color,
                    name = name.collect::<String>(),
                    age_color = color::Fg(color::Rgb(color - 50, color - 50, color - 50)),
                    age = &HumanTime::from(*task.start() - now),
                    reset_color = color::Fg(color::Reset),
                    name_width = name_width
                )?;
            } else {
                write!(
                    f,
                    "[{idx:idx_width$}] {name:width$}\t{age}",
                    idx = idx,
                    idx_width = idx_width,
                    name = name.collect::<String>(),
                    age = &HumanTime::from(*task.start() - now),
                    width = name_width
                )?;
            }

            if let Some(focused) = self.focused {
                if focused == idx {
                    writeln!(f)?;
                    write!(f, "{}", &task)?;
                }
            }

            if idx < len - 1 {
                writeln!(f)?
            }

            color += 100 as u8 / len as u8;
        }
        Ok(())
    }
}
