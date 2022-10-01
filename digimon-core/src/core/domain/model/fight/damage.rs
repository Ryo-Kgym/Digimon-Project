use crate::core::domain::model::fight::effect::Effects;
use crate::core::domain::model::fight::effect::EffectType::AttackMultiply;
use crate::core::domain::model::status::attack::Attack;

#[derive(Debug, PartialEq)]
pub struct Damage {
    pub(crate) value: i32,
}

pub struct DamageBuilder {
    value: i32,
    effects: Effects,
}

impl DamageBuilder {
    pub fn new() -> Self {
        DamageBuilder {
            value: 0,
            effects: Effects::build(),
        }
    }

    pub fn attack(self, attack: Attack) -> Self {
        DamageBuilder {
            value: self.value + attack.value,
            effects: self.effects,
        }
    }

    pub fn effects(self, effects: Effects) -> Self {
        DamageBuilder {
            value: self.value,
            effects: self.effects.append(effects),
        }
    }

    pub fn build(self) -> Damage {
        let mut value = self.value;

        for effect in &self.effects.effects {
            match effect.effect_type {
                AttackMultiply(magnification) => value = Self::calc_value(value, magnification)
            }
        }

        return Damage {
            value,
        };
    }

    fn calc_value(value: i32, magnification: f64) -> i32 {
        (value as f64 * magnification) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::fight::damage::{Damage, DamageBuilder};
    use crate::core::domain::model::fight::effect::Effects;
    use crate::core::domain::model::status::attack::Attack;
    use crate::core::domain::model::status::attribute::Attribute::{DATA, VACCINE, VIRUS};

    #[test]
    fn test_damage_builder() {
        let attack = Attack {
            value: 200,
            effects: Effects::build(),
        };

        let actual = DamageBuilder::new()
            .attack(attack)
            .effects(Effects::of(VIRUS, DATA))
            .effects(Effects::of(VACCINE, VIRUS))
            .build();

        let expected = Damage {
            value: 800,
        };

        assert_eq!(actual, expected)
    }
}
