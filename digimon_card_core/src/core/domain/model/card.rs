use crate::core::domain::model::character::digimon::Digimon;
use crate::core::domain::model::character::digimon::level3_vaccine::{fetch_agu_mon_parameter};
use crate::core::domain::model::fight::effect::Effects;

pub mod deck;

#[derive(Debug, PartialEq, Clone)]
pub enum Card {
	// デジモン
	AguMon,                           // アグモン
	// アイテム
	RecoveryFloppy,                   // 回復フロッピー
	AttackPlugin,                     // 攻撃プラグイン
}

impl Card {
    pub fn get_effects(&self) -> Effects {
        match self {
            Card::RecoveryFloppy => { Effects::of_recovery_effects(300) }
            Card::AttackPlugin => { Effects::of_attack_plus(100) }
            _ => { Effects::empty() }
        }
    }

    pub fn get_digimon(&self) -> Option<Digimon> {
        match self {
            Card::AguMon => { Some(fetch_agu_mon_parameter()) }
            _ => { None }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::card::Card;
    use crate::core::domain::model::character::digimon::level3_vaccine::fetch_agu_mon_parameter;
    use crate::core::domain::model::fight::effect::Effects;

    #[test]
    fn test_get_effects() {
        let actual_list = vec![
            Card::AguMon,
            Card::RecoveryFloppy,
            Card::AttackPlugin,
        ];

        let expected_list = vec![
            Effects::empty(),
            Effects::of_recovery_effects(300),
            Effects::of_attack_plus(100),
        ];

        for i in 0..actual_list.len() {
            assert_eq!(actual_list[i].get_effects(), expected_list[i])
        }
    }

    #[test]
    fn test_get_digimon() {
        let actual_list = vec![
            Card::AguMon,
            Card::RecoveryFloppy,
            Card::AttackPlugin,
        ];

        let expected_list = vec![
            Some(fetch_agu_mon_parameter()),
            None,
            None,
        ];

        for i in 0..actual_list.len() {
            assert_eq!(actual_list[i].get_digimon(), expected_list[i])
        }
    }
}
