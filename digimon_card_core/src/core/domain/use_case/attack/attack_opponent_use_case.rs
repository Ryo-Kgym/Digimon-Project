use crate::core::domain::model::fight::damage::DamageBuilder;
use crate::core::domain::model::fight::effect::Effects;
use crate::core::domain::model::status::attack::Attack;
use crate::core::domain::model::status::attribute::Attribute;
use crate::core::domain::model::status::hit_point::HitPoint;

pub fn attack_opponent(input: AttackOpponentInput) -> AttackOpponentOutput {
    let attribute_effects = Effects::of(input.my_attribute, input.opponent_attribute);
    let damage = DamageBuilder::new()
        .attack(input.my_attack)
        .effects(attribute_effects)
        .build();

    let opponent_hit_point = input.opponent_hit_point.damaged(damage);

    AttackOpponentOutput {
        opponent_hit_point
    }
}

#[derive(Debug, PartialEq)]
pub struct AttackOpponentInput {
    pub my_attack: Attack,
    pub my_attribute: Attribute,
    pub opponent_hit_point: HitPoint,
    pub opponent_attribute: Attribute,
}

#[derive(Debug, PartialEq)]
pub struct AttackOpponentOutput {
    pub opponent_hit_point: HitPoint,
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::fight::effect::Effects;
    use crate::core::domain::model::status::attack::Attack;
    use crate::core::domain::model::status::attribute::Attribute::{VACCINE, VIRUS};
    use crate::core::domain::model::status::hit_point::HitPoint;
    use crate::core::domain::use_case::attack::attack_opponent_use_case::{attack_opponent, AttackOpponentInput, AttackOpponentOutput};

    #[test]
    fn test_attack_enemy() {
        let input = AttackOpponentInput {
            my_attack: Attack {
                value: 50,
                effects: Effects::empty(),
            },
            my_attribute: VACCINE,
            opponent_hit_point: HitPoint::value_of(200),
            opponent_attribute: VIRUS,
        };

        let actual = attack_opponent(input);

        let expected = AttackOpponentOutput {
            opponent_hit_point: HitPoint {
                value: 100,
                max: 200,
                min: 0,
            }
        };

        assert_eq!(actual, expected)
    }
}