use mind::storage::local::LocalStorage;
use mind::{Command, Storage};
use std::env;
use std::io::{self, BufRead, Write};
use termion::screen::AlternateScreen;

fn main() -> io::Result<()> {
    let storage = LocalStorage::init()?;
    let mut mind = storage.load()?;
    mind.remind_tasks();

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
            print!("[{}] ", mind.tasks().len());
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
                mind.act(Command::from(statement).expect("missing command"));
            } else {
                mind.act(Command::Push(input.into()));
            }
        }
    }

    print!("{}", &mind);
    storage.save(mind)?;

    Ok(())
}
