use chrono::Duration;

#[derive(Copy, Clone)]
pub enum Productivity {
    Optimal,
    High,
    Normal,
    Low,
    UnProductive,
}

impl Productivity {
    pub fn from_backlog(backlog: Duration) -> Self {
        let hours = backlog.num_hours();

        if hours <= 12 {
            Self::Optimal
        } else if hours <= 24 {
            Self::High
        } else if hours <= 24 * 3 {
            Self::Normal
        } else if hours <= 24 * 7 {
            Self::Low
        } else {
            Self::UnProductive
        }
    }
}
