use crate::core::domain::model::status::attack::Attack;
use crate::HitPoint;

pub fn attack_enemy(input: AttackEnemyInput) -> AttackEnemyOutput {
    let damage = input.my_attack.to_damage();
    let enemy_hit_point = input.enemy_hit_point.damaged(damage);

    AttackEnemyOutput {
        enemy_hit_point
    }
}

#[derive(Debug, PartialEq)]
pub struct AttackEnemyInput {
    my_attack: Attack,
    enemy_hit_point: HitPoint,
}

#[derive(Debug, PartialEq)]
pub struct AttackEnemyOutput {
    enemy_hit_point: HitPoint,
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::fight::effect::Effects;
    use crate::core::domain::model::status::attack::Attack;
    use crate::core::domain::use_case::attack::attack_enemy::{attack_enemy, AttackEnemyInput, AttackEnemyOutput};
    use crate::HitPoint;

    #[test]
    fn test_attack_enemy() {
        let input = AttackEnemyInput {
            my_attack: Attack {
                value: 50,
                effects: Effects::build(),
            },
            enemy_hit_point: HitPoint::build(100),
        };

        let actual = attack_enemy(input);

        let expected = AttackEnemyOutput {
            enemy_hit_point: HitPoint {
                value: 50,
                max: 100,
                min: 0,
            }
        };

        assert_eq!(actual, expected)
    }
}