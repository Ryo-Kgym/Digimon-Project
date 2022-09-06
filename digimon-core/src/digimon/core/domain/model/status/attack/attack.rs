use crate::digimon::core::domain::model::fight::damage::damage::Damage;
use crate::digimon::core::domain::model::fight::effect::effect::Effects;

#[derive(Debug, PartialEq)]
pub struct Attack {
    value: i32,
    effects: Effects,
}

impl Attack {
    pub fn to_damage(self) -> Damage {
        Damage {
            value: self.value
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::digimon::core::domain::model::fight::damage::damage::Damage;
    use crate::digimon::core::domain::model::fight::effect::effect::Effects;
    use crate::digimon::core::domain::model::status::attack::attack::Attack;

    #[test]
    fn test_to_damage() {
        let actual = Attack {
            value: 100,
            effects: Effects::build(),
        }.to_damage();

        let expected = Damage {
            value: 100
        };

        assert_eq!(actual, expected)
    }
}