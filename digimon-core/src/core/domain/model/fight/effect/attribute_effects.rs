use crate::core::domain::model::fight::effect::{Effect, Effects};
use crate::core::domain::model::fight::effect::EffectType::AttackMultiply;
use crate::core::domain::model::status::attribute::Attribute;

impl Effects {
    pub fn of(my_attribute: Attribute,
              enemy_attribute: Attribute) -> Self {
        let mut magnification: i32 = 1;

        if my_attribute.advantage() == enemy_attribute {
            magnification = 2;
        } else if my_attribute.disadvantage() == enemy_attribute {
            magnification = 1 / 2;
        }

        Effects {
            effects: vec![Effect {
                effect_type: AttackMultiply(magnification),
            }]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::fight::effect::{Effect, Effects};
    use crate::core::domain::model::fight::effect::EffectType::AttackMultiply;
    use crate::core::domain::model::status::attribute::Attribute::{DATA, VACCINE, VIRUS};

    #[test]
    fn test_against_advantage() {
        let actual = Effects::of(
            VACCINE, VIRUS,
        );
        let expected = Effects {
            effects: vec![Effect {
                effect_type: AttackMultiply(2),
            }]
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_against_disadvantage() {
        let actual = Effects::of(
            VACCINE, DATA,
        );
        let expected = Effects {
            effects: vec![Effect {
                effect_type: AttackMultiply(1 / 2),
            }]
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_against_else() {
        let actual = Effects::of(
            VACCINE, VACCINE,
        );
        let expected = Effects {
            effects: vec![Effect {
                effect_type: AttackMultiply(1),
            }]
        };

        assert_eq!(actual, expected)
    }
}