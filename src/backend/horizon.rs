use crate::horizon::Horizon;
use crate::horizon::HorizonPriority;
use crate::object::Object;
use crate::point::Point;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorizonPriorityImpl {
    pub(crate) id: Uuid,
    pub(crate) horizon: Horizon,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) priority_point: Option<Point>,
}

impl Object for HorizonPriorityImpl {
    type Id = Uuid;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl HorizonPriority for HorizonPriorityImpl {
    fn horizon(&self) -> &Horizon {
        &self.horizon
    }

    fn priority_point(&self) -> Option<&Point> {
        self.priority_point.as_ref()
    }
}
