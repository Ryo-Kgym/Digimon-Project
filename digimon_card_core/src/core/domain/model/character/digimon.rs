use crate::core::domain::model::fight::effect::{Effects, EffectType};
use crate::core::domain::model::fight::recovery::Recovery;
use crate::core::domain::model::status::attack::Attack;
use crate::core::domain::model::status::attribute::Attribute;
use crate::core::domain::model::status::hit_point::HitPoint;

#[derive(Debug, PartialEq)]
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
        let mut hit_point = self.hit_point;
        let mut primary_attack = self.primary_attack;
        let mut secondary_attack = self.secondary_attack;
        let mut tertiary_attack = self.tertiary_attack;

        for effect in effects.effects {
            match effect.effect_type {
                EffectType::AttackMultiply(magnification) => {
                    primary_attack = primary_attack.multiply(magnification);
                    secondary_attack = secondary_attack.multiply(magnification);
                    tertiary_attack = tertiary_attack.multiply(magnification);
                }

                EffectType::RecoveryType(value) => {
                    let recovery = Recovery { value };
                    hit_point = hit_point.recovered(recovery);
                }
            }
        }

        Digimon {
            name: String::from(self.name),
            attribute: self.attribute,
            hit_point,
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
    use crate::core::domain::model::fight::effect::EffectType::{AttackMultiply, RecoveryType};
    use crate::core::domain::model::status::attack::Attack;
    use crate::core::domain::model::status::attribute::Attribute;
    use crate::core::domain::model::status::hit_point::HitPoint;

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
            primary_attack: Attack { value: 200, effects: Effects::empty() },
            secondary_attack: Attack { value: 100, effects: Effects::empty() },
            tertiary_attack: Attack { value: 50, effects: Effects::empty() },
        };

        let effects = Effects {
            effects: vec![
                Effect { effect_type: RecoveryType(200) },
                Effect { effect_type: AttackMultiply(2.0 as f64) },
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
            primary_attack: Attack { value: 400, effects: Effects::empty() },
            secondary_attack: Attack { value: 200, effects: Effects::empty() },
            tertiary_attack: Attack { value: 100, effects: Effects::empty() },
        };

        assert_eq!(actual, expected)
    }
}