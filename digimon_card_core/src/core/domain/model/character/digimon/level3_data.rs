use crate::core::domain::model::character::digimon::Digimon;
use crate::core::domain::model::status::attack::Attack;
use crate::core::domain::model::status::attribute::Attribute;
use crate::core::domain::model::status::hit_point::HitPoint;

pub fn fetch_gabu_mon_parameter() -> Digimon {
    Digimon {
        name: "ガブモン".to_string(),
        attribute: Attribute::DATA,
        hit_point: HitPoint::value_of(700),
        primary_attack: Attack::value_of(250),
        secondary_attack: Attack::value_of(180),
        tertiary_attack: Attack::value_of(90),
    }
}