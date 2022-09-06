use crate::digimon::core::domain::status::attack::attack::Attack;
use crate::digimon::core::domain::status::attribute::attribute::Attribute;
use crate::HitPoint;

#[derive(Debug, PartialEq)]
pub struct Digimon {
    name: String,
    attribute: Attribute,
    hit_point: HitPoint,
    primary_attack: Attack,
    secondary_attack: Attack,
    tertiary_attack: Attack,
}