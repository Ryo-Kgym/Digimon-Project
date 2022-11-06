use crate::core::domain::model::card::Card;
use crate::core::domain::model::character::digimon::Digimon;
use crate::core::domain::model::fight::battle::result::BattleResult;
use crate::core::domain::model::status::attack::AttackOrdinal;

// TODO First, Second Attack Rule
pub fn battle(input: BattleInput) -> BattleOutput {
    let effected_my_digimon = match input.my_digimon_card.get_digimon() {
        None => { panic!("This card is not a Digimon Card!!") }

        Some(digimon) => {
            digimon.hit_point(input.my_hit_point_value)
                .obtain_effects(input.my_option_card.get_effects())
        }
    };

    let effected_opponent_digimon = match input.opponent_digimon_card.get_digimon() {
        None => { panic!("This card is not a Digimon Card!!") }

        Some(digimon) => {
            digimon.hit_point(input.opponent_hit_point_value)
                .obtain_effects(input.opponent_option_card.get_effects())
        }
    };

    let opponent_digimon = effected_my_digimon
        .attack(&input.my_attack_ordinal, &effected_opponent_digimon);
    let my_digimon = effected_opponent_digimon
        .attack(&input.opponent_attack_ordinal, &effected_my_digimon);

    let battle_result = BattleResult::of(&my_digimon, &opponent_digimon);

    BattleOutput {
        my_digimon,
        opponent_digimon,
        battle_result,
    }
}

#[derive(PartialEq, Debug)]
pub struct BattleInput {
    my_digimon_card: Card,
    my_hit_point_value: i32,
    my_option_card: Card,
    my_attack_ordinal: AttackOrdinal,
    opponent_digimon_card: Card,
    opponent_hit_point_value: i32,
    opponent_option_card: Card,
    opponent_attack_ordinal: AttackOrdinal,
}

#[derive(PartialEq, Debug)]
pub struct BattleOutput {
    my_digimon: Digimon,
    opponent_digimon: Digimon,
    battle_result: BattleResult,
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::card::Card;
    use crate::core::domain::model::character::digimon::Digimon;
    use crate::core::domain::model::fight::battle::result::BattleResult;
    use crate::core::domain::model::status::attack::{Attack, AttackOrdinal};
    use crate::core::domain::model::status::attribute::Attribute;
    use crate::core::domain::model::status::hit_point::HitPoint;
    use crate::core::domain::phase::battle::{battle, BattleInput, BattleOutput};

    #[test]
    fn test_battle() {
        let actual = battle(BattleInput {
            my_digimon_card: Card::AguMon,
            my_hit_point_value: 590,
            my_option_card: Card::AttackPlugin,
            my_attack_ordinal: AttackOrdinal::PRIMARY,
            opponent_digimon_card: Card::PicoDevilMon,
            opponent_hit_point_value: 500,
            opponent_option_card: Card::RecoveryFloppy,
            opponent_attack_ordinal: AttackOrdinal::SECONDARY,
        });
        let expected = BattleOutput {
            my_digimon: Digimon {
                name: "アグモン".to_string(),
                attribute: Attribute::VACCINE,
                hit_point: HitPoint {
                    value: 465,
                    max: 600,
                    min: 0,
                },
                primary_attack: Attack::value_of(400), // 800
                secondary_attack: Attack::value_of(300),
                tertiary_attack: Attack::value_of(200),
            },
            opponent_digimon: Digimon {
                name: "ピコデビモン".to_string(),
                attribute: Attribute::VIRUS,
                hit_point: HitPoint {
                    value: 0,
                    max: 550,
                    min: 0,
                },
                primary_attack: Attack::value_of(290),
                secondary_attack: Attack::value_of(250), // 125
                tertiary_attack: Attack::value_of(120),
            },
            battle_result: BattleResult::WIN,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    #[should_panic]
    fn error_my_digimon_card() {
        let _actual = battle(BattleInput {
            my_digimon_card: Card::RecoveryFloppy,
            my_hit_point_value: 590,
            my_option_card: Card::AttackPlugin,
            my_attack_ordinal: AttackOrdinal::PRIMARY,
            opponent_digimon_card: Card::PicoDevilMon,
            opponent_hit_point_value: 500,
            opponent_option_card: Card::RecoveryFloppy,
            opponent_attack_ordinal: AttackOrdinal::PRIMARY,
        });
    }

    #[test]
    #[should_panic]
    fn error_opponent_digimon_card() {
        let _actual = battle(BattleInput {
            my_digimon_card: Card::AguMon,
            my_hit_point_value: 590,
            my_option_card: Card::AttackPlugin,
            my_attack_ordinal: AttackOrdinal::PRIMARY,
            opponent_digimon_card: Card::RecoveryFloppy,
            opponent_hit_point_value: 500,
            opponent_option_card: Card::RecoveryFloppy,
            opponent_attack_ordinal: AttackOrdinal::PRIMARY,
        });
    }
}