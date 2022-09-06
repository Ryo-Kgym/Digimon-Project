use crate::digimon::core::domain::fight::damage::damage::Damage;

#[derive(Debug, PartialEq)]
pub struct Attack {
    value: i32,
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
    use crate::digimon::core::domain::fight::damage::damage::Damage;
    use crate::digimon::core::domain::status::attack::attack::Attack;

    #[test]
    fn test_to_damage() {
        let actual = Attack {
            value: 100
        }.to_damage();

        let expected = Damage {
            value: 100
        };

        assert_eq!(actual, expected)
    }
}