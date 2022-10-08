use juniper::{
    GraphQLInputObject,
    GraphQLObject,
};

use digimon_card_core::core::domain::model::fight::effect::Effects;
use digimon_card_core::core::domain::model::status::attack::Attack;
use digimon_card_core::core::domain::model::status::attribute::Attribute;
use digimon_card_core::core::domain::model::status::hit_point::HitPoint;
use digimon_card_core::core::domain::use_case::attack::be_attacked::{BeAttackedInput, BeAttackedOutput};

#[derive(GraphQLInputObject)]
pub struct BeAttackedRequest {
    pub my_hit_point_value: i32,
    pub enemy_attack_value: i32,
}

impl BeAttackedRequest {
    pub fn to_be_attacked_input(&self) -> BeAttackedInput {
        BeAttackedInput {
            enemy_attack: Attack {
                value: self.enemy_attack_value,
                effects: Effects::empty(),
            },
            enemy_attribute: Attribute::VACCINE,
            my_hit_point: HitPoint {
                value: self.my_hit_point_value,
                max: 100,
                min: 0,
            },
            my_attribute: Attribute::VIRUS,
        }
    }
}

#[derive(GraphQLObject)]
#[derive(Debug, PartialEq)]
pub struct BeAttackedResponse {
    pub my_hit_point_value: i32,
}

impl BeAttackedResponse {
    pub fn of(source: BeAttackedOutput) -> Self {
        BeAttackedResponse {
            my_hit_point_value: source.my_hit_point.value
        }
    }
}

#[cfg(test)]
mod tests {
    use digimon_card_core::core::domain::model::fight::effect::Effects;
    use digimon_card_core::core::domain::model::status::attack::Attack;
    use digimon_card_core::core::domain::model::status::attribute::Attribute;
    use digimon_card_core::core::domain::model::status::hit_point::HitPoint;
    use digimon_card_core::core::domain::use_case::attack::be_attacked::{BeAttackedInput, BeAttackedOutput};

    use crate::api::dto::graphql::fight::be_attacked::{BeAttackedRequest, BeAttackedResponse};

    #[test]
    fn test_to_be_attacked_input() {
        let source = BeAttackedRequest {
            my_hit_point_value: 100,
            enemy_attack_value: 50,
        };

        let actual = source.to_be_attacked_input();

        let expected = BeAttackedInput {
            enemy_attack: Attack {
                value: 50,
                effects: Effects::empty(),
            },
            enemy_attribute: Attribute::VACCINE,
            my_hit_point: HitPoint {
                value: 100,
                max: 100,
                min: 0,
            },
            my_attribute: Attribute::VIRUS,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_of() {
        let source = BeAttackedOutput {
            my_hit_point: HitPoint {
                value: 50,
                max: 100,
                min: 0,
            }
        };

        let actual = BeAttackedResponse::of(source);

        let expected = BeAttackedResponse {
            my_hit_point_value: 50
        };

        assert_eq!(actual, expected);
    }
}