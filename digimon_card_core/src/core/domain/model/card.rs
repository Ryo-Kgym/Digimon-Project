use crate::core::domain::model::character::digimon::Digimon;
use crate::core::domain::model::character::digimon::level3_data::fetch_gabu_mon_parameter;
use crate::core::domain::model::character::digimon::level3_vaccine::fetch_agu_mon_parameter;
use crate::core::domain::model::character::digimon::level3_virus::fetch_pico_devil_mon_parameter;
use crate::core::domain::model::fight::effect::Effects;

pub mod deck;

#[derive(Debug, PartialEq, Clone)]
pub enum Card {
	// デジモン
	AguMon,                           // アグモン
	GabuMon,                          // ガブモン
	PicoDevilMon,                     // ピコデビモン
	// アイテム
	RecoveryFloppy,                   // 回復フロッピー
	AttackPlugin,                     // 攻撃プラグイン
}

impl Card {
    pub fn get_effects(&self) -> Effects {
        match self {
            Card::AguMon => { Effects::empty() }
            Card::GabuMon => { Effects::empty() }
            Card::PicoDevilMon => { Effects::empty() }
            Card::RecoveryFloppy => { Effects::of_recovery_effects(300) }
            Card::AttackPlugin => { Effects::of_attack_plus(100) }
        }
    }

    pub fn get_digimon(&self) -> Option<Digimon> {
        match self {
            Card::AguMon => { Some(fetch_agu_mon_parameter()) }
            Card::GabuMon => { Some(fetch_gabu_mon_parameter()) }
            Card::PicoDevilMon => { Some(fetch_pico_devil_mon_parameter()) }
            Card::RecoveryFloppy => { None }
            Card::AttackPlugin => { None }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::card::Card;
    use crate::core::domain::model::character::digimon::level3_data::fetch_gabu_mon_parameter;
    use crate::core::domain::model::character::digimon::level3_vaccine::fetch_agu_mon_parameter;
    use crate::core::domain::model::character::digimon::level3_virus::fetch_pico_devil_mon_parameter;
    use crate::core::domain::model::fight::effect::Effects;

    #[test]
    fn test_get() {
        let actual_list = vec![
            Card::AguMon,
            Card::GabuMon,
            Card::PicoDevilMon,
            Card::RecoveryFloppy,
            Card::AttackPlugin,
        ];

        let effects_expected_list = vec![
            Effects::empty(),
            Effects::empty(),
            Effects::empty(),
            Effects::of_recovery_effects(300),
            Effects::of_attack_plus(100),
        ];

        let digimon_expected_list = vec![
            Some(fetch_agu_mon_parameter()),
            Some(fetch_gabu_mon_parameter()),
            Some(fetch_pico_devil_mon_parameter()),
            None,
            None,
        ];

        for i in 0..actual_list.len() {
            assert_eq!(actual_list[i].get_effects(), effects_expected_list[i]);
            assert_eq!(actual_list[i].get_digimon(), digimon_expected_list[i]);
        }
    }
}
