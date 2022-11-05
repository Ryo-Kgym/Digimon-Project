use crate::core::domain::model::character::digimon::Digimon;
use crate::core::domain::model::status::attack::Attack;
use crate::core::domain::model::status::attribute::Attribute;
use crate::core::domain::model::status::hit_point::HitPoint;

pub fn fetch_pico_devil_mon_parameter() -> Digimon {
    Digimon {
        name: "ピコデビモン".to_string(),
        attribute: Attribute::VIRUS,
        hit_point: HitPoint::value_of(550),
        primary_attack: Attack::value_of(290),
        secondary_attack: Attack::value_of(250),
        tertiary_attack: Attack::value_of(120),
    }
}
