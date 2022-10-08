use digimon_card_core::core::domain::use_case::attack::attack_enemy::attack_enemy;
use digimon_card_core::core::domain::use_case::attack::be_attacked::be_attacked;

use crate::api::config::graphql::schema::MutationRoot;
use crate::api::dto::graphql::attack_enemy::{
    AttackEnemyRequest,
    AttackEnemyResponse,
};
use crate::api::dto::graphql::be_attacked::{BeAttackedRequest, BeAttackedResponse};

#[juniper::graphql_object]
impl MutationRoot {
    #[graphql(description = "My Digimon attacks the enemy.")]
    fn attack_enemy(request: AttackEnemyRequest) -> AttackEnemyResponse {
        let input = request.to_attack_enemy_input();
        AttackEnemyResponse::of(attack_enemy(input))
    }

    #[graphql(description = "My Digimon is attacked by the enemy.")]
    fn be_attacked(request: BeAttackedRequest) -> BeAttackedResponse {
        let input = request.to_be_attacked_input();
        BeAttackedResponse::of(be_attacked(input))
    }
}
