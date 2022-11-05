use juniper::GraphQLInputObject;

use digimon_card_core::core::domain::model::fight::effect::Effects;
use digimon_card_core::core::domain::model::status::attack::Attack;
use digimon_card_core::core::domain::model::status::attribute::Attribute;
use digimon_card_core::core::domain::model::status::hit_point::HitPoint;
use digimon_card_core::core::domain::use_case::attack::attack_opponent_use_case::{
    AttackOpponentInput,
    AttackOpponentOutput,
};

use crate::api::dto::graphql::status::hit_point::HitPoint as HitPointGraphql;

#[derive(GraphQLInputObject)]
pub struct AttackEnemyRequest {
    pub my_attack_value: i32,
    pub enemy_hit_point_value: i32,
}

impl AttackEnemyRequest {
    pub fn to_attack_enemy_input(&self) -> AttackOpponentInput {
        AttackOpponentInput {
            my_attack: Attack {
                value: self.my_attack_value,
                effects: Effects::empty(),
            },
            my_attribute: Attribute::VACCINE,
            opponent_hit_point: HitPoint {
                value: self.enemy_hit_point_value,
                max: 100,
                min: 0,
            },
            opponent_attribute: Attribute::VIRUS,
        }
    }
}

pub fn to_hit_point(output: AttackOpponentOutput) -> HitPointGraphql {
    HitPointGraphql {
        value: *&output.opponent_hit_point.value,
    }
}

#[cfg(test)]
mod tests {
    use digimon_card_core::core::domain::model::fight::effect::Effects;
    use digimon_card_core::core::domain::model::status::attack::Attack;
    use digimon_card_core::core::domain::model::status::attribute::Attribute;
    use digimon_card_core::core::domain::model::status::hit_point::HitPoint;
    use digimon_card_core::core::domain::use_case::attack::attack_opponent_use_case::{
        AttackOpponentInput,
        AttackOpponentOutput,
    };

    use crate::api::dto::graphql::fight::attack_enemy::{AttackEnemyRequest, to_hit_point};
    use crate::api::dto::graphql::status::hit_point::HitPoint as HitPointGraphql;

    #[test]
    fn test_to_attack_enemy_input() {
        let source = AttackEnemyRequest {
            my_attack_value: 50,
            enemy_hit_point_value: 70,
        };

        let actual = source.to_attack_enemy_input();

        let expected = AttackOpponentInput {
            my_attack: Attack {
                value: 50,
                effects: Effects::empty(),
            },
            my_attribute: Attribute::VACCINE,
            opponent_hit_point: HitPoint {
                value: 70,
                max: 100,
                min: 0,
            },
            opponent_attribute: Attribute::VIRUS,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_to_hit_point() {
        let source = AttackOpponentOutput {
            opponent_hit_point: HitPoint {
                value: 50,
                max: 100,
                min: 0,
            }
        };

        let actual = to_hit_point(source);

        let expected = HitPointGraphql {
            value: 50
        };

        assert_eq!(actual, expected)
    }
}