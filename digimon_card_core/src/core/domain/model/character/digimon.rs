use crate::core::domain::model::status::attack::Attack;
use crate::core::domain::model::status::attribute::Attribute;
use crate::core::domain::model::status::hit_point::HitPoint;

#[derive(Debug, PartialEq)]
pub struct Digimon {
    name: String,
    attribute: Attribute,
    hit_point: HitPoint,
    primary_attack: Attack,
    secondary_attack: Attack,
    tertiary_attack: Attack,
}