use digimon_card_core::core::domain::use_case::attack::attack_opponent_use_case::attack_opponent;

use crate::api::config::graphql::schema::MutationRoot;
use crate::api::dto::graphql::fight::attack_enemy::{AttackEnemyRequest, to_hit_point as to_hit_point_attack_enemy};
use crate::api::dto::graphql::fight::be_attacked::{BeAttackedRequest, to_hit_point as to_hit_point_be_attacked};
use crate::api::dto::graphql::status::hit_point::HitPoint;

#[juniper::graphql_object]
impl MutationRoot {
    #[graphql(description = "My Digimon attacks the enemy.")]
    fn attack_enemy(request: AttackEnemyRequest) -> HitPoint {
        let input = request.to_attack_enemy_input();
        to_hit_point_attack_enemy(attack_opponent(input))
    }
}
