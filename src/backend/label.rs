use crate::information::Information;
use crate::label::InformationLabelRelation;
use crate::label::Label;
use crate::label::LabelKind;
use crate::object::Object;
use crate::point::Point;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelImpl {
    pub(crate) id: Uuid,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) kind: Option<LabelKind>,

    pub(crate) value: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) effort_point: Option<Point>,
}

impl Object for LabelImpl {
    type Id = Uuid;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Label for LabelImpl {
    fn kind(&self) -> Option<&LabelKind> {
        self.kind.as_ref()
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn effort_point(&self) -> Option<&Point> {
        self.effort_point.as_ref()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationLabelRelationImpl<I, L>
where
    I: Information,
    L: Label,
{
    pub(crate) id: Uuid,
    pub(crate) information_id: I::Id,
    pub(crate) label_id: L::Id,
}

impl<I, L> Object for InformationLabelRelationImpl<I, L>
where
    I: Information,
    L: Label,
{
    type Id = Uuid;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl<I, L> InformationLabelRelation<I, L> for InformationLabelRelationImpl<I, L>
where
    I: Information,
    L: Label,
{
    fn information_id(&self) -> &I::Id {
        &self.information_id
    }

    fn label_id(&self) -> &L::Id {
        &self.label_id
    }
}
