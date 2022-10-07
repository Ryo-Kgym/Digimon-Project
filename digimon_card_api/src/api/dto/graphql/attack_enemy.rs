use juniper::GraphQLInputObject;
use juniper::GraphQLObject;

use digimon_card_core::core::domain::model::fight::effect::Effects;
use digimon_card_core::core::domain::model::status::attack::Attack;
use digimon_card_core::core::domain::model::status::attribute::Attribute;
use digimon_card_core::core::domain::model::status::hit_point::HitPoint;
use digimon_card_core::core::domain::use_case::attack::attack_enemy::{
    AttackEnemyInput,
    AttackEnemyOutput,
};

#[derive(GraphQLInputObject)]
pub struct AttackEnemyRequest {
    pub my_attack_value: i32,
    pub enemy_hit_point_value: i32,
}

impl AttackEnemyRequest {
    pub fn to_attack_enemy_input(&self) -> AttackEnemyInput {
        AttackEnemyInput {
            my_attack: Attack {
                value: self.my_attack_value,
                effects: Effects::empty(),
            },
            my_attribute: Attribute::VACCINE,
            enemy_hit_point: HitPoint {
                value: self.enemy_hit_point_value,
                max: 100,
                min: 0,
            },
            enemy_attribute: Attribute::VIRUS,
        }
    }
}

#[derive(GraphQLObject)]
#[derive(Debug, PartialEq)]
pub struct AttackEnemyResponse {
    pub enemy_hit_point: i32,
}

impl AttackEnemyResponse {
    pub fn of(output: AttackEnemyOutput) -> Self {
        AttackEnemyResponse {
            enemy_hit_point: output.enemy_hit_point.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use digimon_card_core::core::domain::model::fight::effect::Effects;
    use digimon_card_core::core::domain::model::status::attack::Attack;
    use digimon_card_core::core::domain::model::status::attribute::Attribute;
    use digimon_card_core::core::domain::model::status::hit_point::HitPoint;
    use digimon_card_core::core::domain::use_case::attack::attack_enemy::{
        AttackEnemyInput,
        AttackEnemyOutput,
    };
    use crate::api::dto::graphql::attack_enemy::{AttackEnemyRequest, AttackEnemyResponse};

    #[test]
    fn test_to_attack_enemy_input() {
        let source = AttackEnemyRequest {
            my_attack_value: 50,
            enemy_hit_point_value: 70,
        };

        let actual = source.to_attack_enemy_input();

        let expected = AttackEnemyInput {
            my_attack: Attack {
                value: 50,
                effects: Effects::empty(),
            },
            my_attribute: Attribute::VACCINE,
            enemy_hit_point: HitPoint {
                value: 70,
                max: 100,
                min: 0,
            },
            enemy_attribute: Attribute::VIRUS,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_of() {
        let source = AttackEnemyOutput {
            enemy_hit_point: HitPoint {
                value: 90,
                max: 100,
                min: 0,
            }
        };

        let actual = AttackEnemyResponse::of(source);

        let expected = AttackEnemyResponse {
            enemy_hit_point: 90,
        };

        assert_eq!(actual, expected)
    }
}