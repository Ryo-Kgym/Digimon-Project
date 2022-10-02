use crate::core::domain::model::fight::damage::DamageBuilder;
use crate::core::domain::model::fight::effect::Effects;
use crate::core::domain::model::status::attack::Attack;
use crate::core::domain::model::status::attribute::Attribute;
use crate::HitPoint;

pub fn be_attacked(input: BeAttackedInput) -> BeAttackedOutput {
    let attribute_effects = Effects::of(input.enemy_attribute, input.my_attribute);
    let damage = DamageBuilder::new()
        .attack(input.enemy_attack)
        .effects(attribute_effects)
        .build();
    let my_hit_point = input.my_hit_point.damaged(damage);

    BeAttackedOutput {
        my_hit_point
    }
}

#[derive(Debug, PartialEq)]
pub struct BeAttackedInput {
    enemy_attack: Attack,
    enemy_attribute: Attribute,
    my_hit_point: HitPoint,
    my_attribute: Attribute,
}

#[derive(Debug, PartialEq)]
pub struct BeAttackedOutput {
    my_hit_point: HitPoint,
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::fight::effect::Effects;
    use crate::core::domain::model::status::attack::Attack;
    use crate::core::domain::model::status::attribute::Attribute::{VACCINE, VIRUS};
    use crate::core::domain::use_case::attack::be_attacked::{be_attacked, BeAttackedInput, BeAttackedOutput};
    use crate::HitPoint;

    #[test]
    fn test_be_attacked() {
        let input = BeAttackedInput {
            enemy_attack: Attack {
                value: 50,
                effects: Effects::empty(),
            },
            enemy_attribute: VIRUS,
            my_hit_point: HitPoint::build(100),
            my_attribute: VACCINE,
        };

        let actual = be_attacked(input);

        let expected = BeAttackedOutput {
            my_hit_point: HitPoint {
                value: 75,
                max: 100,
                min: 0,
            }
        };

        assert_eq!(actual, expected)
    }
}