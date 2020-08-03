pub enum Command {
    Push(String),
    Pop(usize),
    PopLast,
    Continue(usize),
    Edit(usize),
    EditLast,
    EditReminders,
    Get(usize),
    GetLast,
    Remind(usize),
    RemindLast,
}

impl<'a> Command {
    pub fn from<I>(mut statement: I) -> Option<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        match statement.next() {
            Some("g") | Some("get") => statement.next().map_or(Some(Self::GetLast), |arg| {
                arg.parse::<usize>()
                    .map_or(None, |num| Some(Self::Get(num)))
            }),

            Some("p") | Some("pop") => statement.next().map_or(Some(Self::PopLast), |arg| {
                arg.parse::<usize>()
                    .map_or(None, |num| Some(Self::Pop(num)))
            }),

            Some("r") | Some("remind") => statement.next().map_or(Some(Self::RemindLast), |arg| {
                arg.parse::<usize>()
                    .map_or(None, |num| Some(Self::Remind(num)))
            }),

            Some("e") | Some("edit") => {
                statement
                    .next()
                    .map_or(Some(Self::EditLast), |arg| match arg {
                        "r" | "reminders" => Some(Self::EditReminders),
                        arg => arg
                            .parse::<usize>()
                            .map_or(None, |num| Some(Self::Edit(num))),
                    })
            }

            Some(arg) => arg
                .parse::<usize>()
                .map_or(None, |num| Some(Self::Continue(num))),

            _ => None,
        }
    }
}
