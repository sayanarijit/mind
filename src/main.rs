use chrono::{DateTime, Utc};
use colored::Colorize;
use dirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct Task {
    start: DateTime<Utc>,
    name: String,
}

impl PartialEq for Task {
    fn eq(&self, other: &Task) -> bool {
        self.name == other.name
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
        write!(f, "⦿  {}", self.name.cyan().bold())
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Mind {
    #[serde(default)]
    tasks: Vec<Task>,
}

impl Mind {
    fn push(&mut self, task: Task) {
        self.tasks.push(task)
    }
    fn pop(&mut self) -> Option<Task> {
        self.tasks.pop()
    }
}

trait Storage {
    fn load(&self) -> io::Result<Mind>;
    fn save(&self, stack: Mind) -> io::Result<()>;
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

        let stack_file_path = local_storage.join(Path::new("stack.json"));
        if !stack_file_path.exists() {
            let file = File::create(&stack_file_path)?;
            serde_json::to_writer(&file, &Mind::default()).unwrap();
        };

        Ok(Self {
            path: stack_file_path,
        })
    }
}

impl Storage for LocalStorage {
    fn load(&self) -> io::Result<Mind> {
        let stack: Mind = serde_json::from_reader(BufReader::new(&File::open(&self.path)?))?;
        return Ok(stack);
    }
    fn save(&self, stack: Mind) -> io::Result<()> {
        serde_json::to_writer(File::create(&self.path)?, &stack).expect("failed to save file.");
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut stdout = std::io::stdout();
    let stdin = io::stdin();
    let storage = LocalStorage::init()?;
    let mut stack = storage.load()?;
    let mut handle = stdin.lock();
    let mut read_input = false;
    let mut args = env::args();

    args.next();

    let maybe_sub = args.next();
    match maybe_sub {
        Some(sub) => match sub.trim() {
            "pop" => {
                stack.pop();
            }
            _ => {
                eprintln!("invalid sub command: {}", sub);
            }
        },
        None => {
            read_input = true;
        }
    }

    stack.tasks.iter().for_each(|task| {
        println!("{}", &task);
    });

    if read_input {
        loop {
            let mut buffer = String::new();

            print!("⦿  ");
            stdout.flush()?;

            handle.read_line(&mut buffer)?;

            if buffer.trim().chars().count() == 0 {
                break;
            }

            let len = stack.tasks.len();

            if let Some((_task, idx)) = stack
                .tasks
                .iter()
                .zip(0..)
                .filter(|(task, _idx)| task.name.trim() == buffer.trim())
                .next()
            {
                stack.tasks.swap(idx, len - 1);
            } else {
                stack.push(Task::new(buffer.trim().to_string()));
            }
        }
    }

    storage.save(stack)?;

    Ok(())
}
