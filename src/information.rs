use crate::object::Object;
use crate::point::Point;
use chrono::NaiveDateTime;
use std::fmt::Debug;

pub trait Information: Object + Sized + Clone + Debug {
    // Capture: Capture any information that comes your way
    fn title(&self) -> &str;
    fn body(&self) -> Option<&str>;
    fn captured_at(&self) -> &NaiveDateTime;

    // Clarify: Clarify the steps required and expected outcome
    fn outcome(&self) -> Option<&str>;
    fn steps(&self) -> &[String];
    fn clarified_at(&self) -> Option<&NaiveDateTime>;

    // Organize: Assign labels to estimate the effort required
    fn organized_at(&self) -> Option<&NaiveDateTime>;

    // Reflect: Estimate how important it is to different horizons of focus
    fn life_importance(&self) -> Option<&Point>;
    fn longterm_vision_importance(&self) -> Option<&Point>;
    fn yearly_goals_importance(&self) -> Option<&Point>;
    fn accountabilities_importance(&self) -> Option<&Point>;
    fn current_projects_importance(&self) -> Option<&Point>;
    fn current_actions_importance(&self) -> Option<&Point>;
    fn reflected_at(&self) -> Option<&NaiveDateTime>;

    // Engage: Based on the estimations and calculated priority, engage.
    fn calculated_priority(&self) -> Option<i32>;
    fn steps_done(&self) -> &[String];
    fn last_step_done_at(&self) -> Option<&NaiveDateTime>;
    fn finished_at(&self) -> Option<&NaiveDateTime>;
    fn archived_at(&self) -> Option<&NaiveDateTime>;
}
