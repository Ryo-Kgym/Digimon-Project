use crate::core::domain::model::card::Card;
use crate::core::domain::model::character::digimon::Digimon;

#[derive(Debug, PartialEq)]
pub struct UseCardInput {
    pub my_card: Card,
    pub my_digimon: Digimon,
    pub enemy_digimon: Digimon,
}

#[derive(Debug, PartialEq)]
pub struct UseCardOutput {
    pub my_digimon: Digimon,
    pub enemy_digimon: Digimon,
}

pub fn use_card(input: UseCardInput) -> UseCardOutput {
    let my_effects = input.my_card.get_effects();
    let my_digimon = input.my_digimon.obtain_effects(my_effects);

    UseCardOutput {
        my_digimon,
        enemy_digimon: input.enemy_digimon,
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::card::Card;
    use crate::core::domain::model::character::digimon::Digimon;
    use crate::core::domain::model::status::attack::Attack;
    use crate::core::domain::model::status::attribute::Attribute;
    use crate::core::domain::model::status::hit_point::HitPoint;
    use crate::core::domain::use_case::card::use_card::{use_card, UseCardInput, UseCardOutput};

    #[test]
    fn test_use_card() {
        let input = UseCardInput {
            my_card: Card::RecoveryFloppy,
            my_digimon: Digimon {
                name: "アグモン".to_string(),
                attribute: Attribute::VACCINE,
                hit_point: HitPoint {
                    value: 300,
                    max: 500,
                    min: 0,
                },
                primary_attack: Attack::value_of(300),
                secondary_attack: Attack::value_of(200),
                tertiary_attack: Attack::value_of(100),
            },
            enemy_digimon: Digimon {
                name: "ピコデビモン".to_string(),
                attribute: Attribute::VIRUS,
                hit_point: HitPoint {
                    value: 300,
                    max: 500,
                    min: 0,
                },
                primary_attack: Attack::value_of(300),
                secondary_attack: Attack::value_of(200),
                tertiary_attack: Attack::value_of(100),
            },
        };
        let actual = use_card(input);

        let expected = UseCardOutput {
            my_digimon: Digimon {
                name: "アグモン".to_string(),
                attribute: Attribute::VACCINE,
                hit_point: HitPoint {
                    value: 500,
                    max: 500,
                    min: 0,
                },
                primary_attack: Attack::value_of(300),
                secondary_attack: Attack::value_of(200),
                tertiary_attack: Attack::value_of(100),
            },
            enemy_digimon: Digimon {
                name: "ピコデビモン".to_string(),
                attribute: Attribute::VIRUS,
                hit_point: HitPoint {
                    value: 300,
                    max: 500,
                    min: 0,
                },
                primary_attack: Attack::value_of(300),
                secondary_attack: Attack::value_of(200),
                tertiary_attack: Attack::value_of(100),
            },
        };

        assert_eq!(actual, expected)
    }
}