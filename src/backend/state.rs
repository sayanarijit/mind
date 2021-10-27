use crate::backend::horizon::HorizonPriorityImpl;
use crate::backend::information::InformationImpl;
use crate::backend::label::InformationLabelRelationImpl;
use crate::backend::label::LabelImpl;
use tui_input::Input;

#[derive(Copy, Clone, Debug)]
pub enum Step {
    // Capture: Capture any information that comes your way
    Capture,

    // Clarify: Clarify the steps required and expected outcome
    Clarify,

    // Organize: Assign labels to estimate the effort required
    Organize,

    // Reflect: Estimate how important it is to different horizons of focus
    Reflect,

    // Engage: Based on the estimations and calculated priority, engage.
    Engage,
}

impl Default for Step {
    fn default() -> Self {
        Self::Capture
    }
}

impl Step {
    pub fn all() -> Vec<Step> {
        vec![
            Self::Capture,
            Self::Clarify,
            Self::Reflect,
            Self::Organize,
            Self::Engage,
        ]
    }

    pub fn next(self) -> Option<Self> {
        match self {
            Self::Capture => Some(Self::Clarify),
            Self::Clarify => Some(Self::Organize),
            Self::Organize => Some(Self::Reflect),
            Self::Reflect => Some(Self::Engage),
            Self::Engage => None,
        }
    }
}

#[derive(Default)]
pub struct State {
    pub(crate) current_step: Step,
    pub(crate) horizon_priority_list: Vec<HorizonPriorityImpl>,
    pub(crate) information_list: Vec<InformationImpl>,
    pub(crate) label_list: Vec<LabelImpl>,
    pub(crate) information_label_relation_list:
        Vec<InformationLabelRelationImpl<InformationImpl, LabelImpl>>,
    pub(crate) input: Option<Input>,
}
