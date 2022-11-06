use crate::core::domain::model::character::digimon::Digimon;

#[derive(PartialEq, Debug)]
pub enum BattleResult {
    WIN,
    LOSE,
    CONTINUE,
}

impl BattleResult {
    pub fn of(my_digimon: &Digimon, opponent_digimon: &Digimon) -> Self {
        if my_digimon.dead() { BattleResult::LOSE } else if opponent_digimon.dead() { BattleResult::WIN } else { BattleResult::CONTINUE }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::character::digimon::Digimon;
    use crate::core::domain::model::fight::battle::result::BattleResult;
    use crate::core::domain::model::status::attack::Attack;
    use crate::core::domain::model::status::attribute::Attribute;
    use crate::core::domain::model::status::hit_point::HitPoint;

    #[test]
    fn test_of_win() {
        let actual = BattleResult::of(
            &Digimon {
                name: "アグモン".to_string(),
                attribute: Attribute::VACCINE,
                hit_point: HitPoint::value_of(600),
                primary_attack: Attack::value_of(300),
                secondary_attack: Attack::value_of(200),
                tertiary_attack: Attack::value_of(100),
            },
            &Digimon {
                name: "ガブモン".to_string(),
                attribute: Attribute::DATA,
                hit_point: HitPoint::value_of(0),
                primary_attack: Attack::value_of(300),
                secondary_attack: Attack::value_of(200),
                tertiary_attack: Attack::value_of(100),
            });

        assert_eq!(actual, BattleResult::WIN)
    }

    #[test]
    fn test_of_lose() {
        let actual = BattleResult::of(
            &Digimon {
                name: "アグモン".to_string(),
                attribute: Attribute::VACCINE,
                hit_point: HitPoint::value_of(0),
                primary_attack: Attack::value_of(300),
                secondary_attack: Attack::value_of(200),
                tertiary_attack: Attack::value_of(100),
            },
            &Digimon {
                name: "ガブモン".to_string(),
                attribute: Attribute::DATA,
                hit_point: HitPoint::value_of(1),
                primary_attack: Attack::value_of(300),
                secondary_attack: Attack::value_of(200),
                tertiary_attack: Attack::value_of(100),
            });

        assert_eq!(actual, BattleResult::LOSE)
    }

    #[test]
    fn test_of_continue() {
        let actual = BattleResult::of(
            &Digimon {
                name: "アグモン".to_string(),
                attribute: Attribute::VACCINE,
                hit_point: HitPoint::value_of(1),
                primary_attack: Attack::value_of(300),
                secondary_attack: Attack::value_of(200),
                tertiary_attack: Attack::value_of(100),
            },
            &Digimon {
                name: "ガブモン".to_string(),
                attribute: Attribute::DATA,
                hit_point: HitPoint::value_of(1),
                primary_attack: Attack::value_of(300),
                secondary_attack: Attack::value_of(200),
                tertiary_attack: Attack::value_of(100),
            });

        assert_eq!(actual, BattleResult::CONTINUE)
    }
}