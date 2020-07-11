use mind::storage::local::LocalStorage;
use mind::{Command, Mind, Storage};
use std::env;
use std::io::{self, BufRead, Write};
use termion::screen::AlternateScreen;

// TODO proper error handling
fn run() -> io::Result<()> {
    let storage = LocalStorage::init()?;
    let mut mind = storage.load()?;
    mind.remind_tasks();

    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        if args.get(0).unwrap() == "--version" {
            println!("{}", Mind::version());
            std::process::exit(0);
        } else if args.get(0).unwrap() == "--help" {
            println!("mind - A productive mind");
            println!();
            println!("ARGS:");
            println!("  --version      \tPrint the binary version");
            println!("  --help         \tPrint this help menu");
            println!();
            println!("SUB COMMANDS:");
            println!("  {{num}}        \t\tContinue with the task at {{num}} position");
            println!("  pop            \t(alias: p) Pop the last task from mind");
            println!("  pop {{num}}    \t\t(alias: p {{num}}) Pop the task at {{num}} position");
            println!("  edit           \t(alias: e) Edit the last task in mind");
            println!("  edit {{num}}   \t\t(alias: e {{num}}) Edit the task at {{num}} position");
            std::process::exit(0);
        } else if let Some(command) = Command::from(args.iter().map(|x| x.trim())) {
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

                if let Some(command) = Command::from(statement) {
                    mind.act(command);
                }
            } else {
                mind.act(Command::Push(input.into()));
            }
        }
    }

    print!("{}", &mind);
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
