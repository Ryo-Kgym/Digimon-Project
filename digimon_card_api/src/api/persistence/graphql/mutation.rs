use async_graphql::{Object, SimpleObject};

use digimon_card_core::{HitPoint, recover_hit_point, RecoverHitPointInput, Recovery};

pub struct DigimonMutation;

#[Object]
impl DigimonMutation {
    async fn recover_hit_point(&self, item_value: i32) -> HitPointViewModel {
        let h = recover_hit_point(RecoverHitPointInput {
            hit_point: HitPoint {
                value: 50,
                min: 0,
                max: 200,
            },
            recovery: Recovery {
                value: item_value,
            },
        });

        HitPointViewModel {
            value: h.hit_point.value
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct HitPointViewModel {
    pub value: i32,
}
