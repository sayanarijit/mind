pub enum Command {
    Push(String),
    Pop(usize),
    PopLast,
    Continue(usize),
    Edit(usize),
    EditLast,
}

impl<'a> Command {
    pub fn from<I: Iterator<Item = &'a str>>(mut statement: I) -> Option<Self> {
        match statement.next() {
            Some("p") | Some("pop") => Some(statement.next().map_or(Self::PopLast, |arg| {
                Self::Pop(arg.parse().expect("invalid argument"))
            })),
            Some("e") | Some("edit") => Some(statement.next().map_or(Self::EditLast, |arg| {
                Self::Edit(arg.parse().expect("invalid argument"))
            })),
            Some(num) => Some(Self::Continue(num.parse().expect("invalid argument"))),
            _ => None,
        }
    }
}
