use juniper::GraphQLInputObject;

use digimon_card_core::core::domain::model::fight::effect::Effects;
use digimon_card_core::core::domain::model::status::attack::Attack;
use digimon_card_core::core::domain::model::status::attribute::Attribute;
use digimon_card_core::core::domain::model::status::hit_point::HitPoint;
use digimon_card_core::core::domain::use_case::attack::be_attacked::{
    BeAttackedInput,
    BeAttackedOutput,
};

use crate::api::dto::graphql::status::hit_point::HitPoint as HitPointGraphql;

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

pub fn to_hit_point(output: BeAttackedOutput) -> HitPointGraphql {
    HitPointGraphql {
        value: *&output.my_hit_point.value
    }
}

#[cfg(test)]
mod tests {
    use digimon_card_core::core::domain::model::fight::effect::Effects;
    use digimon_card_core::core::domain::model::status::attack::Attack;
    use digimon_card_core::core::domain::model::status::attribute::Attribute;
    use digimon_card_core::core::domain::model::status::hit_point::HitPoint;
    use digimon_card_core::core::domain::use_case::attack::be_attacked::{BeAttackedInput, BeAttackedOutput};

    use crate::api::dto::graphql::fight::be_attacked::{BeAttackedRequest, to_hit_point};
    use crate::api::dto::graphql::status::hit_point::HitPoint as HitPointGraphql;

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
    fn test_to_hit_point() {
        let source = BeAttackedOutput {
            my_hit_point: HitPoint {
                value: 50,
                max: 100,
                min: 0,
            }
        };

        let actual = to_hit_point(source);

        let expected = HitPointGraphql {
            value: 50
        };

        assert_eq!(actual, expected);
    }
}