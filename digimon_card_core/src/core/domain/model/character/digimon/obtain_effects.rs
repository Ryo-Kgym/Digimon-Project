use crate::core::domain::model::character::digimon::Digimon;
use crate::core::domain::model::character::digimon::obtain_effects::attack_multiply::AttackMultiplyObtainEffects;
use crate::core::domain::model::character::digimon::obtain_effects::attack_plus::AttackPlusObtainEffects;
use crate::core::domain::model::character::digimon::obtain_effects::recovery_type::RecoveryTypeObtainEffects;
use crate::core::domain::model::fight::effect::{Effects, EffectType};

pub mod attack_multiply;
pub mod attack_plus;
pub mod recovery_type;

pub trait ObtainEffects {
    fn apply(self, digimon: Digimon, value: usize) -> Digimon;
}

pub fn allocate(source: Digimon, effects: Effects) -> Digimon {
    let mut digimon = source;

    for effect in effects.effects {
        match effect.effect_type {
            EffectType::AttackMultiply(value) => {
                digimon = AttackMultiplyObtainEffects
                    .apply(digimon, value as usize)
            }

            EffectType::RecoveryType(value) => {
                digimon = RecoveryTypeObtainEffects
                    .apply(digimon, value as usize)
            }

            EffectType::AttackPlus(value) => {
                digimon = AttackPlusObtainEffects
                    .apply(digimon, value as usize)
            }
        }
    }
    digimon
}