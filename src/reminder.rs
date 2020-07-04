use chrono::{DateTime, Local, Weekday};
use serde::{Deserialize, Serialize};

pub static REMINDER_EXAMPLES: &str = r###"
# This reminder will disappear once executed.

- name: Test reminder once on 10 July 2020, at 8 am IST
  when: "2020-07-10T08:00:00+05:30"
  repeat: Never

# The following reminders will reschedule themselves.
# And Will keep re-scheduling for all the reminders you've missed.

- name: "Test reminder everyday at 10:30 pm IST"
  when: "2020-07-10T10:30:00+05:30"
  repeat: EveryDay

- name: "Test reminder every other day at 10:30 pm IST"
  when: "2020-07-10T10:30:00+05:30"
  repeat:
    EveryNthDay: 2

- name: Test reminder every week at 11 am IST
  when: "2020-07-10T11:00:00+05:30"
  repeat: EveryWeek

- name: Test reminder every 3rd week at 11 am IST
  when: "2020-07-10T11:00:00+05:30"
  repeat:
    EveryNthWeek: 3

- name: "Test reminder every saturday and sunday at 9:15 am IST"
  when: "2020-07-10T09:15:00+05:30"
  repeat:
    Weekdays:
      - Sat
      - Sun

- name: "Test reminder every 2nd saturday at 9:15 am IST"
  when: "2020-07-10T09:15:00+05:30"
  repeat:
    EveryNthWeekday:
      n: 2
      weekday: Sat
"###;

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

    pub fn examples() -> Vec<Reminder> {
        let lines: Vec<&str> = REMINDER_EXAMPLES
            .lines()
            .collect();
        let tmpl = lines.join("\n");
        // println!("{}", &tmpl);
        let reminders: Vec<Reminder> =
            serde_yaml::from_str(tmpl.trim()).expect("invalid reminders template");
        reminders
    }
}
