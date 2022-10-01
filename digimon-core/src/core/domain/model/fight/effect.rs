pub(crate) mod attribute_effects;

#[derive(Debug, PartialEq)]
pub enum EffectType {
    AttackMultiply(i32),
}

#[derive(Debug, PartialEq)]
pub struct Effect {
    pub(crate) effect_type: EffectType,
}

#[derive(Debug, PartialEq)]
pub struct Effects {
    pub(crate) effects: Vec<Effect>,
}

impl Effects {
    pub fn build() -> Effects {
        let effects = Vec::new();

        Effects {
            effects
        }
    }
}