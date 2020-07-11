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
            Some("p") | Some("pop") => statement.next().map_or(Some(Self::PopLast), |arg| {
                arg.parse::<usize>()
                    .map_or(None, |num| Some(Self::Pop(num)))
            }),

            Some("e") | Some("edit") => statement.next().map_or(Some(Self::EditLast), |arg| {
                arg.parse::<usize>()
                    .map_or(None, |num| Some(Self::Edit(num)))
            }),

            Some(arg) => arg
                .parse::<usize>()
                .map_or(None, |num| Some(Self::Continue(num))),

            _ => None,
        }
    }
}
