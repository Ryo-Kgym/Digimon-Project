use crate::core::domain::model::status::attribute::Attribute::{DATA, VACCINE, VIRUS};

#[derive(Debug, PartialEq)]
pub enum Attribute {
    VACCINE,
    DATA,
    VIRUS,
}

impl Attribute {
    pub fn advantage(self) -> Attribute {
        match self {
            VACCINE => VIRUS,
            DATA => VACCINE,
            VIRUS => DATA,
        }
    }

    pub fn disadvantage(self) -> Attribute {
        match self {
            VACCINE => DATA,
            DATA => VIRUS,
            VIRUS => VACCINE,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::status::attribute::Attribute::{DATA, VACCINE, VIRUS};

    #[test]
    fn test_advantage() {
        assert_eq!(VACCINE.advantage(), VIRUS);
        assert_eq!(DATA.advantage(), VACCINE);
        assert_eq!(VIRUS.advantage(), DATA);
    }

    #[test]
    fn test_disadvantage() {
        assert_eq!(VACCINE.disadvantage(), DATA);
        assert_eq!(DATA.disadvantage(), VIRUS);
        assert_eq!(VIRUS.disadvantage(), VACCINE);
    }
}