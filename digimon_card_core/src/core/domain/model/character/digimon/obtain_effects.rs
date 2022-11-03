use crate::core::domain::model::character::digimon::Digimon;
use crate::core::domain::model::character::digimon::obtain_effects::attack_multiply::AttackMultiplyObtainEffects;
use crate::core::domain::model::character::digimon::obtain_effects::attack_plus::AttackPlusObtainEffects;
use crate::core::domain::model::character::digimon::obtain_effects::recovery_type::RecoveryTypeObtainEffects;
use crate::core::domain::model::fight::effect::{Effects, EffectType};

pub mod attack_multiply;
pub mod attack_plus;
pub mod recovery_type;

pub struct EffectsObtainer<T: ObtainEffects> {
    obtain_effects: T,
}

impl<T: ObtainEffects> EffectsObtainer<T> {
    fn new(obtain_effects: T) -> Self {
        Self { obtain_effects }
    }

    fn apply(self, digimon: Digimon, value: usize) -> Digimon {
        self.obtain_effects.apply(digimon, value)
    }
}

pub trait ObtainEffects {
    fn apply(self, digimon: Digimon, value: usize) -> Digimon;
}

pub fn allocate_effects(source: Digimon, effects: Effects) -> Digimon {
    let mut digimon = source;

    for effect in effects.effects {
        match effect.effect_type {
            EffectType::AttackMultiply(value) => {
                digimon = EffectsObtainer::new(AttackMultiplyObtainEffects)
                    .apply(digimon, value as usize)
            }

            EffectType::RecoveryType(value) => {
                digimon = EffectsObtainer::new(RecoveryTypeObtainEffects)
                    .apply(digimon, value as usize)
            }

            EffectType::AttackPlus(value) => {
                digimon = EffectsObtainer::new(AttackPlusObtainEffects)
                    .apply(digimon, value as usize)
            }
        }
    }
    digimon
}