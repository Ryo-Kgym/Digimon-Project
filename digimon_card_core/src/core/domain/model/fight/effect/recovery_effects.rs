use crate::core::domain::model::fight::effect::{Effect, Effects};
use crate::core::domain::model::fight::effect::EffectType::RecoveryType;

impl Effects {
    pub fn of_recovery_effects(recovery_value: i32) -> Self {
        let e = Effect {
            effect_type: RecoveryType(recovery_value)
        };

        Effects {
            effects: vec![e]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::fight::effect::{Effect, Effects};
    use crate::core::domain::model::fight::effect::EffectType::RecoveryType;

    #[test]
    fn test_of_recovery_effects() {
        let actual = Effects::of_recovery_effects(300);

        let expected = Effects {
            effects: vec![Effect {
                effect_type: RecoveryType(300)
            }]
        };

        assert_eq!(actual, expected)
    }
}