use crate::core::domain::model::character::digimon::obtain_effects::allocate_effects;
use crate::core::domain::model::fight::effect::Effects;
use crate::core::domain::model::status::attack::Attack;
use crate::core::domain::model::status::attribute::Attribute;
use crate::core::domain::model::status::hit_point::HitPoint;

pub mod obtain_effects;
pub mod level3_vaccine;

#[derive(Debug, PartialEq, Clone)]
pub struct Digimon {
    pub name: String,
    pub attribute: Attribute,
    pub hit_point: HitPoint,
    pub primary_attack: Attack,
    pub secondary_attack: Attack,
    pub tertiary_attack: Attack,
}

impl Digimon {
    pub fn obtain_effects(self, effects: Effects) -> Self {
        allocate_effects(self, effects)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::character::digimon::Digimon;
    use crate::core::domain::model::fight::effect::{Effect, Effects};
    use crate::core::domain::model::fight::effect::EffectType::{AttackMultiply, RecoveryType};
    use crate::core::domain::model::status::attack::Attack;
    use crate::core::domain::model::status::attribute::Attribute;
    use crate::core::domain::model::status::hit_point::HitPoint;

    // AttackMultiply
    // RecoveryType
    #[test]
    fn test_obtain_effects() {
        let digimon = Digimon {
            name: "アグモン".to_string(),
            attribute: Attribute::VACCINE,
            hit_point: HitPoint {
                value: 300,
                max: 600,
                min: 0,
            },
            primary_attack: Attack::value_of(200),
            secondary_attack: Attack::value_of(100),
            tertiary_attack: Attack::value_of(50),
        };

        let effects = Effects {
            effects: vec![
                Effect { effect_type: RecoveryType(200) },
                Effect { effect_type: AttackMultiply(2.0 as f64) },
                Effect { effect_type: RecoveryType(20) },
            ]
        };

        let actual = digimon.obtain_effects(effects);

        let expected = Digimon {
            name: "アグモン".to_string(),
            attribute: Attribute::VACCINE,
            hit_point: HitPoint {
                value: 520,
                max: 600,
                min: 0,
            },
            primary_attack: Attack::value_of(400),
            secondary_attack: Attack::value_of(200),
            tertiary_attack: Attack::value_of(100),
        };

        assert_eq!(actual, expected)
    }
}