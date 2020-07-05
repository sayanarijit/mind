use chrono::{DateTime, Datelike, Duration, Local, Weekday};
use serde::{Deserialize, Serialize};

pub static REMINDER_EXAMPLES: &str = r###"
# This reminder will disappear once executed.

- name: Test reminder once on 10 July 2020, at 8 am IST
  when: "2020-07-10T08:00:00+05:30"
  repeat: Never

# The following reminders will reschedule themselves.

- name: "Test reminder everyday at 10:30 pm IST"
  when: "2020-07-10T22:30:00+05:30"
  repeat: EveryDay

- name: "Test reminder every other day at 10:30 pm IST"
  when: "2020-07-10T22:30:00+05:30"
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
impl Repeat {
    pub fn when_next(&self, when_last: DateTime<Local>) -> Option<DateTime<Local>> {
        match self {
            Self::Never => None,
            Self::EveryDay => Some(when_last + Duration::days(1)),
            Self::EveryNthDay(days) => Some(when_last + Duration::days(*days as i64)),
            Self::EveryWeek => Some(when_last + Duration::days(7)),
            Self::EveryNthWeek(weeks) => Some(when_last + Duration::days((weeks * 7).into())),
            Self::Weekdays(weekdays) | Self::Weekly(weekdays) => {
                let mut weekday = when_last.weekday().succ();
                let mut days = 1;

                while !weekdays.contains(&weekday) {
                    weekday = weekday.succ();
                    days += 1;
                }

                Some(when_last + Duration::days(days))
            }
            Self::EveryNthWeekday(nthweekday) => {
                let mut weekday = when_last.weekday().succ();
                let mut days = 1;

                while weekday != nthweekday.weekday() {
                    weekday = weekday.succ();
                    days += 1;
                }

                Some(when_last + Duration::days((days + 7 * nthweekday.n()).into()))
            }
        }
    }
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
        let reminders: Vec<Reminder> =
            serde_yaml::from_str(REMINDER_EXAMPLES).expect("invalid reminders template");
        reminders
    }
    pub fn next(&self) -> Option<Self> {
        self.repeat
            .when_next(self.when.clone())
            .map(|when| Self::new(self.name.clone(), when, self.repeat.clone()))
    }
}
