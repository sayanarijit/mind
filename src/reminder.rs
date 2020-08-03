use chrono::{DateTime, Datelike, Duration, Local, Weekday};
use serde::{Deserialize, Serialize};

// Use Reminder::examples()
static REMINDER_EXAMPLES: &str = r###"
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
    fn when_next(&self, when_last: DateTime<Local>) -> Option<DateTime<Local>> {
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

    pub fn when_upcoming(
        &self,
        when_last: DateTime<Local>,
        now: Option<DateTime<Local>>,
    ) -> Option<DateTime<Local>> {
        let now = now.unwrap_or_else(Local::now);

        if let Some(when_next) = self.when_next(when_last) {
            let mut when_next = when_next;

            while when_next <= now {
                when_next = self.when_next(when_next).unwrap();
            }

            return Some(when_next);
        };

        None
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Reminder {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    when: DateTime<Local>,
    repeat: Repeat,
}

impl Reminder {
    pub fn new(
        name: String,
        details: Option<String>,
        when: DateTime<Local>,
        repeat: Repeat,
    ) -> Self {
        Self {
            name,
            details,
            when,
            repeat,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn details(&self) -> &Option<String> {
        &self.details
    }

    pub fn when(&self) -> &DateTime<Local> {
        &self.when
    }

    pub fn repeat(&self) -> &Repeat {
        &self.repeat
    }

    pub fn examples() -> &'static str {
        REMINDER_EXAMPLES
    }

    pub fn upcoming(&self, now: Option<DateTime<Local>>) -> Option<Self> {
        self.repeat.when_upcoming(self.when, now).map(|when| {
            let (name, details) = (self.name.clone(), self.details.clone());
            Self::new(name, details, when, self.repeat.clone())
        })
    }
}
