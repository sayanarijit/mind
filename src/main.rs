use mind::storage::local::LocalStorage;
use mind::{Command, Mind, Storage};
use std::env;
use std::io::{self, BufRead, Write};
use termion::screen::AlternateScreen;

static HELP: &str = r###"
mind - A productive mind

ARGS:
  --version                       Print the binary version
  --help                          Print this help menu

SUB COMMANDS:
  Command        | Aliases   | Action
  ---------------|-----------|------------------------------------------
  {num}          |           | Continue with the task at the given position
  pop            | p         | Pop out the current task
  pop {num}      | p {num}   | Pop out the task at the given position
  edit           | e         | Edit the current task
  edit {num}     | e {num}   | Edit the task at the given position
  edit reminders | e r       | Edit the reminders
  get            | g         | Get details of the current task
  get {num}      | g {num}   | Get details of the task at the given position
  remind         | r         | Turn the current task into a reminder
  remind {num}   | r {num}   | Turn the specified task into a reminder
"###;

// TODO proper error handling
fn run() -> io::Result<()> {
    let storage = LocalStorage::init()?;
    let mut mind = storage.load()?;
    mind.remind_tasks();

    let args: Vec<String> = env::args().skip(1).collect();
    if !args.is_empty() {
        if args.get(0).unwrap() == "--version" {
            println!("{}", Mind::version());
            std::process::exit(0);
        } else if args.get(0).unwrap() == "--help" {
            println!("{}", HELP);
            std::process::exit(0);
        } else if let Some(command) = Command::from(args.iter().map(|x| x.trim())) {
            mind.act(command);
        } else {
            eprintln!("error: invalid sub command: {}", args.join(" "));
            std::process::exit(1);
        }
    } else if atty::is(atty::Stream::Stdout) {
        loop {
            let stdout = std::io::stdout();
            let mut stdout = AlternateScreen::from(stdout);
            let stdin = io::stdin();
            let mut handle = stdin.lock();

            println!("{}", &mind);
            print!("[{}] ", mind.tasks().len());
            stdout.flush()?;

            let mut buffer = String::new();
            handle.read_line(&mut buffer)?;

            let input = buffer.trim();

            if input.chars().count() == 0 {
                break;
            };

            if input.starts_with('/') {
                let statement = input
                    .splitn(2, '/')
                    .nth(1)
                    .expect("missing command")
                    .split(' ');

                if let Some(command) = Command::from(statement) {
                    mind.act(command);
                }
            } else {
                mind.act(Command::Push(input.into()));
            }
        }
    }

    if let Some(focused) = mind.focused() {
        println!("{}", &focused);
    } else {
        println!("{}", &mind);
    }

    storage.save(mind)
}

fn main() {
    run()
        .map_err(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        })
        .unwrap();
}
