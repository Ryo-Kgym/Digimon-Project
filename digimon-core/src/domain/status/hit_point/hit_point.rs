use crate::domain::fight::damage::Damage;

const MIN_VALUE: i32 = 0;

#[derive(Debug, PartialEq)]
pub struct HitPoint {
    value: i32,
    max: i32,
    min: i32,
}

impl HitPoint {
    pub fn build(max: i32) -> Self {
        HitPoint {
            value: max,
            max,
            min: MIN_VALUE,
        }
    }

    pub fn damaged(self, damage: Damage) -> Self  {
        HitPoint {
            value: (self.value - damage.value).max(0),
            max: self.max,
            min: MIN_VALUE,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::fight::damage::Damage;
    use crate::HitPoint;

    #[test]
    fn test_hit_point_build() {
        let actual = HitPoint::build(100);
        let expected = HitPoint {
            value: 100,
            max: 100,
            min: 0,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_damaged_rest() {
        let hit_point = HitPoint::build(100);
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
    fn test_damaged_non_rest() {
        let hit_point = HitPoint::build(100);
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
}