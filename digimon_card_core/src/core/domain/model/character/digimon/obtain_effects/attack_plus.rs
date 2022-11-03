use crate::core::domain::model::character::digimon::Digimon;
use crate::core::domain::model::character::digimon::obtain_effects::ObtainEffects;

pub struct AttackPlusObtainEffects;

impl ObtainEffects for AttackPlusObtainEffects {
    fn apply(self, digimon: Digimon, value: usize) -> Digimon {
        let primary_attack = digimon.primary_attack.add(value as i32);
        let secondary_attack = digimon.secondary_attack.add(value as i32);
        let tertiary_attack = digimon.tertiary_attack.add(value as i32);

        Digimon {
            name: String::from(digimon.name),
            attribute: digimon.attribute,
            hit_point: digimon.hit_point,
            primary_attack,
            secondary_attack,
            tertiary_attack,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::character::digimon::Digimon;
    use crate::core::domain::model::fight::effect::{Effect, Effects};
    use crate::core::domain::model::fight::effect::EffectType::AttackPlus;
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
                Effect { effect_type: AttackPlus(50) },
            ]
        };

        let actual = digimon.obtain_effects(effects);

        let expected = Digimon {
            name: "アグモン".to_string(),
            attribute: Attribute::VACCINE,
            hit_point: HitPoint {
                value: 300,
                max: 600,
                min: 0,
            },
            primary_attack: Attack { value: 250, effects: Effects::empty() },
            secondary_attack: Attack { value: 150, effects: Effects::empty() },
            tertiary_attack: Attack { value: 100, effects: Effects::empty() },
        };

        assert_eq!(actual, expected)
    }
}
