use crate::core::domain::model::fight::effect::Effects;

#[derive(Debug, PartialEq)]
pub struct Attack {
    pub value: i32,
    pub effects: Effects,
}
