#[derive(Debug, PartialEq)]
pub struct Effect {}

#[derive(Debug, PartialEq)]
pub struct Effects {
    effects: Vec<Effect>,
}

impl Effects {
    pub fn build() -> Effects {
        let effects = Vec::new();

        Effects {
            effects
        }
    }
}