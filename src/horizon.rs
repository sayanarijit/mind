use crate::{object::Object, point::Point};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Horizon {
    /// Life purpose and values
    Life = 1,

    /// Long-term visions
    LongTermVision = 2,

    /// 1–2 year goals
    YearlyGoals = 3,

    /// Areas of focus and accountability
    Accountabilities = 4,

    /// Current projects
    CurrentProjects = 5,

    /// Current actions
    CurrentActions = 6,
}

impl Horizon {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Life,
            Self::LongTermVision,
            Self::YearlyGoals,
            Self::Accountabilities,
            Self::CurrentProjects,
            Self::CurrentActions,
        ]
    }

    pub fn zoom_out(&self) -> Option<Self> {
        match self {
            Self::Life => None,
            Self::LongTermVision => Some(Self::Life),
            Self::YearlyGoals => Some(Self::LongTermVision),
            Self::Accountabilities => Some(Self::YearlyGoals),
            Self::CurrentProjects => Some(Self::Accountabilities),
            Self::CurrentActions => Some(Self::CurrentProjects),
        }
    }

    pub fn zoom_in(&self) -> Option<Self> {
        match self {
            Self::Life => Some(Self::LongTermVision),
            Self::LongTermVision => Some(Self::YearlyGoals),
            Self::YearlyGoals => Some(Self::Accountabilities),
            Self::Accountabilities => Some(Self::CurrentProjects),
            Self::CurrentProjects => Some(Self::CurrentActions),
            Self::CurrentActions => None,
        }
    }
}

pub trait HorizonPriority: Object + Sized + Clone + Debug {
    fn horizon(&self) -> &Horizon;
    fn priority_point(&self) -> Option<&Point>;
}
