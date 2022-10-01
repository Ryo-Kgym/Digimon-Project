use crate::core::domain::model::fight::damage::Damage;
use crate::core::domain::model::fight::effect::Effects;

#[derive(Debug, PartialEq)]
pub struct Attack {
    pub value: i32,
    pub effects: Effects,
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
    use crate::core::domain::model::fight::damage::Damage;
    use crate::core::domain::model::fight::effect::Effects;
    use crate::core::domain::model::status::attack::Attack;

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