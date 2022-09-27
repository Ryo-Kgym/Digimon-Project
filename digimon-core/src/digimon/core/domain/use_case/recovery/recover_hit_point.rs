use crate::digimon::core::domain::model::fight::recovery::recovery::Recovery;
use crate::HitPoint;

pub fn recover_hit_point(input: RecoverHitPointInput) -> RecoverHitPointOutput {
    let recovered = input.hit_point.recovered(input.recovery);

    RecoverHitPointOutput {
        hit_point: recovered,
    }
}

#[derive(Debug, PartialEq)]
pub struct RecoverHitPointInput {
    hit_point: HitPoint,
    recovery: Recovery,
}

#[derive(Debug, PartialEq)]
pub struct RecoverHitPointOutput {
    hit_point: HitPoint,
}

#[cfg(test)]
mod tests {
    use crate::digimon::core::domain::model::fight::recovery::recovery::Recovery;
    use crate::digimon::core::domain::use_case::recovery::recover_hit_point::{recover_hit_point, RecoverHitPointInput, RecoverHitPointOutput};
    use crate::HitPoint;

    #[test]
    fn test_recover_hit_point() {
        let input = RecoverHitPointInput {
            hit_point: HitPoint {
                value: 80,
                max: 100,
                min: 0,
            },
            recovery: Recovery {
                value: 30
            },
        };

        let actual = recover_hit_point(input);

        let expected = RecoverHitPointOutput {
            hit_point: HitPoint {
                value: 100,
                max: 100,
                min: 0,
            }
        };

        assert_eq!(actual, expected)
    }
}