use crate::{Command, Productivity, Reminder, Repeat, Task};
use chrono::Duration;
use chrono::Local;
use chrono_humanize::HumanTime;
use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;
use termion::color;
use termion::terminal_size;

// Access it using Mind::version()
static VERSION: &str = "0.7.4";

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

    fn push(&mut self, task: Task) {
        if let Some((_task, idx)) = self
            .tasks
            .iter()
            .zip(0..)
            .find(|(t, _i)| t.name().trim() == task.name().trim())
        {
            let task = self.tasks.remove(idx);
            self.tasks.push(task);
        } else {
            self.tasks.push(task);
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
            .map(|idx| self.tasks.get(idx).map(Some).unwrap_or(None))
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

            self.push(Task::from_reminder(&reminder));

            if let Some(upcoming) = reminder.upcoming(Some(now)) {
                new_reminders.push(upcoming);
            }
        }
        self.reminders = new_reminders;
    }

    /// Total backlog
    pub fn backlog(&self) -> Duration {
        let now = Local::now();
        self.tasks
            .iter()
            .map(|t| (now - *t.start()))
            .fold(Duration::zero(), |x, y| x + y)
    }

    /// Productivity from backlog
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

        process::Command::new(env::var("EDITOR").unwrap_or_else(|_| "vi".into()))
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
            name.trim_start_matches("# ").trim().into(),
            if details.chars().count() > 0 {
                Some(details.into())
            } else {
                None
            },
        );

        fs::remove_file(path)
    }

    fn edit_reminders(&mut self) -> io::Result<()> {
        // TODO: do the same for Task edit?

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

        loop {
            process::Command::new(env::var("EDITOR").unwrap_or_else(|_| "vi".into()))
                .arg(&path)
                .status()
                .expect("failed to open editor");

            let mut file = File::open(&path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            if content.is_empty() {
                break;
            }

            let probably_reminders = serde_yaml::from_str(content.trim());
            match probably_reminders {
                Ok(reminders) => {
                    self.reminders = reminders;
                    break;
                }

                Err(err) => {
                    let updated_lines: Vec<String> = [
                        "# There was an error in the previous attempt",
                        "# ┌─────────────────────────────────────────",
                        format!("{}", err)
                            .lines()
                            .map(|l| format!("# │ ERROR: {}", &l))
                            .collect::<Vec<String>>()
                            .join("\n")
                            .trim(),
                        "# └────────────────────────────────────────────────────────",
                        "# If you want to cancel or quit, just leave this file empty",
                    ]
                    .iter()
                    .map(|l| l.to_string())
                    .chain(
                        content
                            .lines()
                            .map(String::from)
                            .skip_while(|l| l.starts_with("# ")),
                    )
                    .collect();

                    {
                        let mut file = fs::File::create(&path)?;
                        write!(file, "{}", updated_lines.join("\n"))?;
                    }
                }
            };
        }

        fs::remove_file(path)
    }

    /// Turn the specified task into a reminder
    pub fn task_to_reminder(&mut self, index: usize) -> io::Result<()> {
        let task = self.tasks.remove(index);
        let reminder = Reminder::new(
            task.name().clone(),
            task.details().clone(),
            Local::now(),
            Repeat::Never,
        );
        self.reminders.insert(0, reminder);
        self.edit_reminders()
    }

    /// Act based on the given command.
    pub fn act(&mut self, command: Command) {
        self.focused = None;

        match command {
            Command::Push(name) => {
                self.push(Task::new(name));
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
                if !self.tasks.is_empty() {
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
                if !self.tasks.is_empty() {
                    self.edit(self.tasks.len() - 1).expect("failed to edit");
                }
            }
            Command::Remind(index) => {
                if index < self.tasks.len() {
                    self.task_to_reminder(index).expect("failed to edit");
                }
            }

            Command::RemindLast => {
                if !self.tasks.is_empty() {
                    self.task_to_reminder(self.tasks.len() - 1)
                        .expect("failed to edit");
                }
            }

            Command::EditReminders => self.edit_reminders().expect("failed to edit"),
        }
    }
}

impl fmt::Display for Mind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut color = 155u8;
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

            color += 100u8 / len as u8;
        }
        Ok(())
    }
}
