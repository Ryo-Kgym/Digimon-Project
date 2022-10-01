use crate::core::domain::model::fight::damage::DamageBuilder;
use crate::core::domain::model::fight::effect::Effects;
use crate::core::domain::model::status::attack::Attack;
use crate::core::domain::model::status::attribute::Attribute;
use crate::HitPoint;

pub fn attack_enemy(input: AttackEnemyInput) -> AttackEnemyOutput {
    let attribute_effects = Effects::of(input.my_attribute, input.enemy_attribute);
    let damage = DamageBuilder::new()
        .attack(input.my_attack)
        .effects(attribute_effects)
        .build();

    let enemy_hit_point = input.enemy_hit_point.damaged(damage);

    AttackEnemyOutput {
        enemy_hit_point
    }
}

#[derive(Debug, PartialEq)]
pub struct AttackEnemyInput {
    my_attack: Attack,
    my_attribute: Attribute,
    enemy_hit_point: HitPoint,
    enemy_attribute: Attribute,
}

#[derive(Debug, PartialEq)]
pub struct AttackEnemyOutput {
    enemy_hit_point: HitPoint,
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::fight::effect::Effects;
    use crate::core::domain::model::status::attack::Attack;
    use crate::core::domain::model::status::attribute::Attribute::{VACCINE, VIRUS};
    use crate::core::domain::use_case::attack::attack_enemy::{attack_enemy, AttackEnemyInput, AttackEnemyOutput};
    use crate::HitPoint;

    #[test]
    fn test_attack_enemy() {
        let input = AttackEnemyInput {
            my_attack: Attack {
                value: 50,
                effects: Effects::empty(),
            },
            my_attribute: VACCINE,
            enemy_hit_point: HitPoint::build(200),
            enemy_attribute: VIRUS,
        };

        let actual = attack_enemy(input);

        let expected = AttackEnemyOutput {
            enemy_hit_point: HitPoint {
                value: 100,
                max: 200,
                min: 0,
            }
        };

        assert_eq!(actual, expected)
    }
}