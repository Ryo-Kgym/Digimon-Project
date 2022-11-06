use crate::core::domain::model::character::digimon::Digimon;
use crate::core::domain::model::fight::effect::Effects;

#[derive(Debug, PartialEq, Clone)]
pub struct Attack {
    pub value: i32,
    pub effects: Effects,
}

impl Attack {
    pub fn value_of(value: i32) -> Self {
        Attack {
            value,
            effects: Effects::empty()
        }
    }

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

#[derive(PartialEq, Debug)]
pub enum AttackOrdinal {
    PRIMARY,
    SECONDARY,
    TERTIARY,
}

impl AttackOrdinal {
    pub fn to_attack(&self, digimon: &Digimon) -> Attack {
        match &self {
            AttackOrdinal::PRIMARY => { digimon.primary_attack.clone() }
            AttackOrdinal::SECONDARY => { digimon.secondary_attack.clone() }
            AttackOrdinal::TERTIARY => { digimon.tertiary_attack.clone() }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::character::digimon::Digimon;
    use crate::core::domain::model::fight::effect::Effects;
    use crate::core::domain::model::status::attack::{Attack, AttackOrdinal};
    use crate::core::domain::model::status::attribute::Attribute;
    use crate::core::domain::model::status::hit_point::HitPoint;

    #[test]
    fn test_value_of() {
        let actual = Attack::value_of(300);
        let expected = Attack {
            value: 300,
            effects: Effects::empty(),
        };

        assert_eq!(actual, expected)
    }

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

    #[test]
    fn test_ordinal_primary() {
        let digimon = Digimon {
            name: "アグモン".to_string(),
            attribute: Attribute::VACCINE,
            hit_point: HitPoint::value_of(600),
            primary_attack: Attack::value_of(300),
            secondary_attack: Attack::value_of(200),
            tertiary_attack: Attack::value_of(100),
        };

        let actual = AttackOrdinal::PRIMARY.to_attack(&digimon);

        assert_eq!(actual, Attack::value_of(300))
    }

    #[test]
    fn test_ordinal_secondary() {
        let digimon = Digimon {
            name: "アグモン".to_string(),
            attribute: Attribute::VACCINE,
            hit_point: HitPoint::value_of(600),
            primary_attack: Attack::value_of(300),
            secondary_attack: Attack::value_of(200),
            tertiary_attack: Attack::value_of(100),
        };

        let actual = AttackOrdinal::SECONDARY.to_attack(&digimon);

        assert_eq!(actual, Attack::value_of(200))
    }

    #[test]
    fn test_ordinal_tertiary() {
        let digimon = Digimon {
            name: "アグモン".to_string(),
            attribute: Attribute::VACCINE,
            hit_point: HitPoint::value_of(600),
            primary_attack: Attack::value_of(300),
            secondary_attack: Attack::value_of(200),
            tertiary_attack: Attack::value_of(100),
        };

        let actual = AttackOrdinal::TERTIARY.to_attack(&digimon);

        assert_eq!(actual, Attack::value_of(100))
    }
}
