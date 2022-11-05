use crate::core::domain::model::character::digimon::Digimon;
use crate::core::domain::model::status::attack::Attack;
use crate::core::domain::model::status::attribute::Attribute;
use crate::core::domain::model::status::hit_point::HitPoint;

pub fn fetch_agu_mon_parameter() -> Digimon {
    Digimon {
        name: "アグモン".to_string(),
        attribute: Attribute::VACCINE,
        hit_point: HitPoint::value_of(600),
        primary_attack: Attack::value_of(300),
        secondary_attack: Attack::value_of(200),
        tertiary_attack: Attack::value_of(100),
    }
}
