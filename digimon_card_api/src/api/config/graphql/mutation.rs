use digimon_card_core::core::domain::use_case::attack::attack_enemy::attack_enemy;
use digimon_card_core::core::domain::use_case::attack::be_attacked::be_attacked;

use crate::api::config::graphql::schema::MutationRoot;
use crate::api::dto::graphql::fight::attack_enemy::{AttackEnemyRequest, to_hit_point};
use crate::api::dto::graphql::fight::be_attacked::{BeAttackedRequest, BeAttackedResponse};
use crate::api::dto::graphql::status::hit_point::HitPoint;

#[juniper::graphql_object]
impl MutationRoot {
    #[graphql(description = "My Digimon attacks the enemy.")]
    fn attack_enemy(request: AttackEnemyRequest) -> HitPoint {
        let input = request.to_attack_enemy_input();
        to_hit_point(attack_enemy(input))
    }

    #[graphql(description = "My Digimon is attacked by the enemy.")]
    fn be_attacked(request: BeAttackedRequest) -> BeAttackedResponse {
        let input = request.to_be_attacked_input();
        BeAttackedResponse::of(be_attacked(input))
    }
}
