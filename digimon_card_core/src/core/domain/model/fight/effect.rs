pub mod attribute_effects;

#[derive(Debug, PartialEq)]
pub enum EffectType {
    AttackMultiply(f64),
}

#[derive(Debug, PartialEq)]
pub struct Effect {
    pub effect_type: EffectType,
}

#[derive(Debug, PartialEq)]
pub struct Effects {
    pub effects: Vec<Effect>,
}

impl Effects {
    pub fn empty() -> Self {
        let effects = Vec::new();

        Effects {
            effects
        }
    }

    pub fn append(self, mut added: Effects) -> Self {
        let mut effects = self.effects;
        effects.append(&mut added.effects);

        Effects {
            effects
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::fight::effect::{Effect, Effects};
    use crate::core::domain::model::fight::effect::EffectType::AttackMultiply;
    use crate::core::domain::model::status::attribute::Attribute::{DATA, VACCINE, VIRUS};

    #[test]
    fn test_empty() {
        let actual = Effects::empty();
        let expected = Effects {
            effects: Vec::new(),
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_append() {
        let actual = Effects::of(VACCINE, VIRUS)
            .append(Effects::of(DATA, VIRUS));
        let expected = Effects {
            effects: vec![
                Effect {
                    effect_type: AttackMultiply(2.0),
                },
                Effect {
                    effect_type: AttackMultiply(0.5),
                },
            ],
        };

        assert_eq!(actual, expected)
    }
}