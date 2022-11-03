use crate::core::domain::model::fight::effect::{Effect, Effects};
use crate::core::domain::model::fight::effect::EffectType::AttackPlus;

impl Effects {
    pub fn of_attack_plus(value: i32) -> Self {
        return Effects {
            effects: vec![
                Effect { effect_type: AttackPlus(value) }
            ]
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::fight::effect::{Effect, Effects};
    use crate::core::domain::model::fight::effect::EffectType::AttackPlus;

    #[test]
    fn test_of_attack_plus() {
        let actual = Effects::of_attack_plus(100);

        let expected = Effects {
            effects: vec![Effect {
                effect_type: AttackPlus(100)
            }]
        };

        assert_eq!(actual, expected)
    }
}