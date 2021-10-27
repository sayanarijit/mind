use crate::point::Point;
use crate::{information::Information, object::Object};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum LabelKind {
    DoByHours = 1,
    DoByDays = 2,
    DoByWeeks = 3,
    DoByMonths = 4,
    DoByYears = 5,

    StartsAfterHours = 6,
    StartsAfterDays = 7,
    StartsAfterWeeks = 8,
    StartsAfterMonths = 9,
    StartsAfterYears = 10,

    RepeatsAfterHours = 11,
    RepeatsAfterDays = 12,
    RepeatsAfterWeeks = 13,
    RepeatsAfterMonths = 14,
    RepeatsAfterYears = 15,

    RepeatsForHours = 16,
    RepeatsForDays = 17,
    RepeatsForWeeks = 18,
    RepeatsForMonths = 19,
    RepeatsForYears = 20,

    RequiresPerson = 21,
    RequiresLocation = 22,
    RequiresObject = 23,
    RequiresMoney = 24,
    RequiresPhysicalEffort = 25,
    RequiresMentalEffort = 26,
    RequiresTime = 27,
    RequiresSkill = 28,

    DoBeforeFinishing = 29,
    DoAfterFinishing = 30,
}

pub trait Label: Object + Sized + Clone + Debug {
    fn kind(&self) -> Option<&LabelKind>;
    fn value(&self) -> &str;
    fn description(&self) -> Option<&str>;
    fn effort_point(&self) -> Option<&Point>;
}

pub trait InformationLabelRelation<I, L>:
    Object + Sized + Clone + Debug
where
    I: Information,
    L: Label,
{
    fn information_id(&self) -> &I::Id;
    fn label_id(&self) -> &L::Id;
}
