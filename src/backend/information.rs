use crate::information::Information;
use crate::object::Object;
use crate::point::Point;
use chrono::NaiveDateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InformationImpl {
    pub(crate) id: Uuid,

    pub(crate) title: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) body: Option<String>,

    pub(crate) captured_at: NaiveDateTime,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) outcome: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) steps: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) clarified_at: Option<NaiveDateTime>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) organized_at: Option<NaiveDateTime>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) life_importance: Option<Point>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) longterm_vision_importance: Option<Point>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) yearly_goals_importance: Option<Point>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) accountabilities_importance: Option<Point>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) current_projects_importance: Option<Point>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) current_actions_importance: Option<Point>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) reflected_at: Option<NaiveDateTime>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) calculated_priority: Option<i32>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) steps_done: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) last_step_done_at: Option<NaiveDateTime>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) finished_at: Option<NaiveDateTime>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) archived_at: Option<NaiveDateTime>,
}

impl Object for InformationImpl {
    type Id = Uuid;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Information for InformationImpl {
    fn title(&self) -> &str {
        &self.title
    }

    fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    fn captured_at(&self) -> &NaiveDateTime {
        &self.captured_at
    }

    fn outcome(&self) -> Option<&str> {
        self.outcome.as_deref()
    }

    fn steps(&self) -> &[String] {
        &self.steps.as_slice()
    }

    fn clarified_at(&self) -> Option<&NaiveDateTime> {
        self.clarified_at.as_ref()
    }

    fn organized_at(&self) -> Option<&NaiveDateTime> {
        self.organized_at.as_ref()
    }

    fn life_importance(&self) -> Option<&Point> {
        self.life_importance.as_ref()
    }

    fn longterm_vision_importance(&self) -> Option<&Point> {
        self.longterm_vision_importance.as_ref()
    }

    fn yearly_goals_importance(&self) -> Option<&Point> {
        self.yearly_goals_importance.as_ref()
    }

    fn accountabilities_importance(&self) -> Option<&Point> {
        self.accountabilities_importance.as_ref()
    }

    fn current_projects_importance(&self) -> Option<&Point> {
        self.current_actions_importance.as_ref()
    }

    fn current_actions_importance(&self) -> Option<&Point> {
        self.current_actions_importance.as_ref()
    }

    fn reflected_at(&self) -> Option<&NaiveDateTime> {
        self.reflected_at.as_ref()
    }

    fn calculated_priority(&self) -> Option<i32> {
        self.calculated_priority
    }

    fn steps_done(&self) -> &[String] {
        self.steps_done.as_slice()
    }

    fn last_step_done_at(&self) -> Option<&NaiveDateTime> {
        self.last_step_done_at.as_ref()
    }

    fn finished_at(&self) -> Option<&NaiveDateTime> {
        self.finished_at.as_ref()
    }

    fn archived_at(&self) -> Option<&NaiveDateTime> {
        self.archived_at.as_ref()
    }
}

impl InformationImpl {
    pub fn new(text: &str) -> Self {
        let (title, body) = if let Some((title, body)) = text.split_once("\n") {
            (title.to_string(), Some(body.to_string()))
        } else {
            (text.to_string(), None)
        };

        let id = Uuid::new_v4();
        let captured_at = Utc::now().naive_utc();

        Self {
            id,
            title,
            body,
            captured_at,
            outcome: Default::default(),
            steps: Default::default(),
            clarified_at: Default::default(),
            organized_at: Default::default(),
            life_importance: Default::default(),
            longterm_vision_importance: Default::default(),
            yearly_goals_importance: Default::default(),
            accountabilities_importance: Default::default(),
            current_projects_importance: Default::default(),
            current_actions_importance: Default::default(),
            reflected_at: Default::default(),
            calculated_priority: Default::default(),
            steps_done: Default::default(),
            last_step_done_at: Default::default(),
            finished_at: Default::default(),
            archived_at: Default::default(),
        }
    }
}
