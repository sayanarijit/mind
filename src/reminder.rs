use chrono::{DateTime, Local, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct NthWeekday {
    n: u32,
    weekday: Weekday,
}

impl NthWeekday {
    pub fn from(n: u32, weekday: Weekday) -> Self {
        Self { n, weekday }
    }
    pub fn n(&self) -> u32 {
        self.n
    }
    pub fn weekday(&self) -> Weekday {
        self.weekday
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Repeat {
    Never,
    EveryDay,
    EveryNthDay(u32),
    EveryWeek,
    EveryNthWeek(u32),
    Weekly(Vec<Weekday>),
    Weekdays(Vec<Weekday>),
    EveryNthWeekday(NthWeekday),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Reminder {
    name: String,
    when: DateTime<Local>,
    repeat: Repeat,
}

impl Reminder {
    pub fn new(name: String, when: DateTime<Local>, repeat: Repeat) -> Self {
        Self { name, when, repeat }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn when(&self) -> &DateTime<Local> {
        &self.when
    }
    pub fn repeat(&self) -> &Repeat {
        &self.repeat
    }
}
