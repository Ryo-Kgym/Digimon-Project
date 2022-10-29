use crate::core::domain::model::character::digimon::Digimon;
use crate::core::domain::model::character::digimon::obtain_effects::ObtainEffects;
use crate::core::domain::model::fight::recovery::Recovery;

pub struct RecoveryTypeObtainEffects;

impl ObtainEffects for RecoveryTypeObtainEffects {
    fn apply(self, digimon: Digimon, value: usize) -> Digimon {
        let recovery = Recovery { value: value as i32 };
        let hit_point = digimon.hit_point.recovered(recovery);

        Digimon {
            name: String::from(digimon.name),
            attribute: digimon.attribute,
            hit_point,
            primary_attack: digimon.primary_attack,
            secondary_attack: digimon.secondary_attack,
            tertiary_attack: digimon.tertiary_attack,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::character::digimon::Digimon;
    use crate::core::domain::model::fight::effect::{Effect, Effects};
    use crate::core::domain::model::fight::effect::EffectType::RecoveryType;
    use crate::core::domain::model::status::attack::Attack;
    use crate::core::domain::model::status::attribute::Attribute;
    use crate::core::domain::model::status::hit_point::HitPoint;

    #[test]
    fn test_apply() {
        let digimon = Digimon {
            name: "アグモン".to_string(),
            attribute: Attribute::VACCINE,
            hit_point: HitPoint {
                value: 300,
                max: 600,
                min: 0,
            },
            primary_attack: Attack { value: 200, effects: Effects::empty() },
            secondary_attack: Attack { value: 100, effects: Effects::empty() },
            tertiary_attack: Attack { value: 50, effects: Effects::empty() },
        };

        let effects = Effects {
            effects: vec![
                Effect { effect_type: RecoveryType(200) },
            ]
        };

        let actual = digimon.obtain_effects(effects);

        let expected = Digimon {
            name: "アグモン".to_string(),
            attribute: Attribute::VACCINE,
            hit_point: HitPoint {
                value: 500,
                max: 600,
                min: 0,
            },
            primary_attack: Attack { value: 200, effects: Effects::empty() },
            secondary_attack: Attack { value: 100, effects: Effects::empty() },
            tertiary_attack: Attack { value: 50, effects: Effects::empty() },
        };

        assert_eq!(actual, expected)
    }
}
