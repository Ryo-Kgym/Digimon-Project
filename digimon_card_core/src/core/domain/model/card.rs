use crate::core::domain::model::fight::effect::Effects;

#[derive(PartialEq)]
pub enum Card {
	// アイテム
	RecoveryFloppy,                   // 回復フロッピー
	AttackPlugin,                     // 攻撃プラグイン
}

impl Card {
	pub fn get_effects(&self) -> Effects {
		match self {
			Card::RecoveryFloppy => { Effects::of_recovery_effects(300) }
			// TODO
			Card::AttackPlugin => { Effects::empty() }
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::core::domain::model::card::Card;
	use crate::core::domain::model::fight::effect::Effects;

	#[test]
	fn test_get_effects() {
		let actual_list = vec![
			Card::RecoveryFloppy,
			Card::AttackPlugin,
		];

		let expected_list = vec![
			Effects::of_recovery_effects(300),
			Effects::empty(),
		];

		for i in 0..actual_list.len() {
			assert_eq!(actual_list[i].get_effects(), expected_list[i])
		}
	}
}
