use crate::digimon::core::domain::model::status::attack::attack::Attack;
use crate::HitPoint;

pub fn be_attacked(input: BeAttackedInput) -> BeAttackedOutput {
    let damage = input.enemy_attack.to_damage();
    let enemy_hit_point = input.my_hit_point.damaged(damage);

    BeAttackedOutput {
        my_hit_point: enemy_hit_point
    }
}

#[derive(Debug, PartialEq)]
pub struct BeAttackedInput {
    enemy_attack: Attack,
    my_hit_point: HitPoint,
}

#[derive(Debug, PartialEq)]
pub struct BeAttackedOutput {
    my_hit_point: HitPoint,
}

#[cfg(test)]
mod tests {
    use crate::digimon::core::domain::model::fight::effect::effect::Effects;
    use crate::digimon::core::domain::model::status::attack::attack::Attack;
    use crate::digimon::core::domain::use_case::attack::be_attacked::{be_attacked, BeAttackedInput, BeAttackedOutput};
    use crate::HitPoint;

    #[test]
    fn test_be_attacked() {
        let input = BeAttackedInput {
            enemy_attack: Attack {
                value: 50,
                effects: Effects::build(),
            },
            my_hit_point: HitPoint::build(100),
        };

        let actual = be_attacked(input);

        let expected = BeAttackedOutput {
            my_hit_point: HitPoint {
                value: 50,
                max: 100,
                min: 0,
            }
        };

        assert_eq!(actual, expected)
    }
}