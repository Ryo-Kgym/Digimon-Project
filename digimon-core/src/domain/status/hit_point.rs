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
}

#[cfg(test)]
mod tests {
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
}