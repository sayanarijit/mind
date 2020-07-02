use chrono::{DateTime, Utc};
use dirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use termion::color;
use termion::screen::AlternateScreen;

enum Command {
    Continue(usize),
    Pop(usize),
    PopLast,
    // Edit(usize),
}

impl<'a> Command {
    fn from<I: Iterator<Item = &'a str>>(mut statement: I) -> Option<Self> {
        match statement.next() {
            Some("c") | Some("continue") => Some(Self::Continue(
                statement
                    .next()
                    .expect("missing argument")
                    .parse()
                    .expect("invalid argument"),
            )),
            Some("p") | Some("pop") => Some(statement.next().map_or(Self::PopLast, |arg| {
                Self::Pop(arg.parse().expect("invalid argument"))
            })),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Task {
    start: DateTime<Utc>,
    name: String,
}

impl PartialEq for Task {
    fn eq(&self, other: &Task) -> bool {
        self.name.trim() == other.name.trim()
    }
}

impl Task {
    fn new(name: String) -> Self {
        Self {
            name,
            start: Utc::now(),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Mind {
    #[serde(default)]
    tasks: Vec<Task>,
}

impl Mind {
    fn push(&mut self, name: &str) {
        if let Some((_task, idx)) = self
            .tasks
            .iter()
            .zip(0..)
            .filter(|(task, _idx)| task.name.trim() == name)
            .next()
        {
            let task = self.tasks.remove(idx);
            self.tasks.push(task);
        } else {
            self.tasks.push(Task::new(name.to_string()));
        }
    }
    fn pop(&mut self) -> Option<Task> {
        self.tasks.pop()
    }

    fn act(&mut self, command: Command) {
        match command {
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

        for (task, idx) in self.tasks.iter().zip(0..) {
            writeln!(
                f,
                "[{}] {}{}{}",
                idx,
                color::Fg(color::Rgb(color - 70, color - 30, color)),
                &task.name,
                color::Fg(color::Reset)
            )?;
            color += 100 as u8 / len as u8;
        }
        Ok(())
    }
}

trait Storage {
    fn load(&self) -> io::Result<Mind>;
    fn save(&self, mind: Mind) -> io::Result<()>;
}

struct LocalStorage {
    path: PathBuf,
}

impl LocalStorage {
    fn init() -> io::Result<Self> {
        let local_storage = dirs::home_dir()
            .expect("failed go get home directory")
            .join(Path::new(".mind"));
        if !local_storage.exists() {
            fs::create_dir(&local_storage)?;
        };

        let mind_file_path = local_storage.join(Path::new("mind.json"));
        if !mind_file_path.exists() {
            let file = File::create(&mind_file_path)?;
            serde_json::to_writer(&file, &Mind::default()).unwrap();
        };

        Ok(Self {
            path: mind_file_path,
        })
    }
}

impl Storage for LocalStorage {
    fn load(&self) -> io::Result<Mind> {
        let mind: Mind = serde_json::from_reader(BufReader::new(&File::open(&self.path)?))?;
        return Ok(mind);
    }
    fn save(&self, mind: Mind) -> io::Result<()> {
        serde_json::to_writer(File::create(&self.path)?, &mind).expect("failed to save file.");
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let storage = LocalStorage::init()?;
    let mut mind = storage.load()?;
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() > 0 {
        if let Some(command) = Command::from(args.iter().map(|x| x.trim())) {
            mind.act(command);
        } else {
            eprintln!("error: invalid sub command: {}", args.get(0).unwrap());
            std::process::exit(1);
        }
    } else {
        loop {
            let stdout = std::io::stdout();
            let mut stdout = AlternateScreen::from(stdout);
            let stdin = io::stdin();
            let mut handle = stdin.lock();

            print!("{}", &mind);
            print!("[{}] ", mind.tasks.len());
            stdout.flush()?;

            let mut buffer = String::new();
            handle.read_line(&mut buffer)?;

            let input = buffer.trim();

            if input.chars().count() == 0 {
                break;
            }

            if input.chars().next() == Some('/') {
                let statement = input
                    .splitn(2, '/')
                    .skip(1)
                    .next()
                    .expect("missing command")
                    .split(' ');
                if let Some(command) = Command::from(statement) {
                    mind.act(command)
                }
            } else {
                mind.push(input);
            }
        }
    }

    print!("{}", &mind);
    storage.save(mind)?;

    Ok(())
}
