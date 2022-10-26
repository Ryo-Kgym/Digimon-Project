use crate::core::domain::model::fight::effect::Effects;

#[derive(Debug, PartialEq)]
pub struct Attack {
    pub value: i32,
    pub effects: Effects,
}

impl Attack {
    pub fn add(self, value: i32) -> Self {
        Attack {
            value: self.value + value,
            effects: self.effects,
        }
    }

    pub fn multiply(self, magnification: f64) -> Self {
        Attack {
            value: ((self.value as f64) * magnification) as i32,
            effects: self.effects,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::fight::effect::Effects;
    use crate::core::domain::model::status::attack::Attack;

    #[test]
    fn test_add() {
        let actual = Attack {
            value: 300,
            effects: Effects::empty(),
        }
            .add(200);

        let expected = Attack {
            value: 500,
            effects: Effects::empty(),
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_multiply() {
        let actual = Attack {
            value: 300,
            effects: Effects::empty(),
        }
            .multiply(2 as f64);

        let expected = Attack {
            value: 600,
            effects: Effects::empty(),
        };

        assert_eq!(actual, expected)
    }
}
