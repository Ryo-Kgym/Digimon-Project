use crate::core::domain::model::fight::damage::Damage;
use crate::core::domain::model::fight::recovery::Recovery;

const MIN_VALUE: i32 = 0;

#[derive(Debug, PartialEq, Clone)]
pub struct HitPoint {
    pub value: i32,
    pub max: i32,
    pub min: i32,
}

impl HitPoint {
    pub fn value_of(max: i32) -> Self {
        HitPoint {
            value: max,
            max,
            min: MIN_VALUE,
        }
    }

    pub fn value(self, value: i32) -> Self {
        HitPoint {
            value,
            max: self.max,
            min: self.min,
        }
    }

    pub fn damaged(self, damage: Damage) -> Self {
        HitPoint {
            value: (self.value - damage.value).max(MIN_VALUE),
            max: self.max,
            min: MIN_VALUE,
        }
    }

    pub fn recovered(self, recovery: Recovery) -> Self {
        HitPoint {
            value: (self.value + recovery.value).min(self.max),
            max: self.max,
            min: MIN_VALUE,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::model::fight::damage::Damage;
    use crate::core::domain::model::fight::recovery::Recovery;
    use crate::core::domain::model::status::hit_point::HitPoint;

    #[test]
    fn test_value_of() {
        let actual = HitPoint::value_of(100);
        let expected = HitPoint {
            value: 100,
            max: 100,
            min: 0,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_value() {
        let actual = HitPoint::value_of(1000)
            .value(500);
        let expected = HitPoint {
            value: 500,
            max: 1000,
            min: 0,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_damaged_greater_than_min() {
        let hit_point = HitPoint::value_of(100);
        let damage = Damage {
            value: 20,
        };
        let actual = hit_point.damaged(damage);

        let expected = HitPoint {
            value: 80,
            max: 100,
            min: 0,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_damaged_less_than_min() {
        let hit_point = HitPoint::value_of(100);
        let damage = Damage {
            value: 200,
        };
        let actual = hit_point.damaged(damage);

        let expected = HitPoint {
            value: 0,
            max: 100,
            min: 0,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_recovered_less_than_max() {
        let hit_point = HitPoint {
            value: 50,
            max: 100,
            min: 0,
        };
        let recovery = Recovery {
            value: 20,
        };

        let actual = hit_point.recovered(recovery);
        let expected = HitPoint {
            value: 70,
            max: 100,
            min: 0,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_recovered_greater_than_max() {
        let hit_point = HitPoint {
            value: 50,
            max: 100,
            min: 0,
        };
        let recovery = Recovery {
            value: 80,
        };

        let actual = hit_point.recovered(recovery);
        let expected = HitPoint {
            value: 100,
            max: 100,
            min: 0,
        };

        assert_eq!(actual, expected)
    }
}