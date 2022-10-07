use digimon_card_core::core::domain::use_case::attack::attack_enemy::attack_enemy;

use crate::api::config::graphql::schema::MutationRoot;
use crate::api::dto::graphql::attack_enemy::{
    AttackEnemyRequest,
    AttackEnemyResponse,
};

#[juniper::graphql_object]
impl MutationRoot {
    #[graphql(description = "My Digimon attacks the enemy.")]
    fn attack_enemy(request: AttackEnemyRequest) -> AttackEnemyResponse {
        let input = request.to_attack_enemy_input();
        AttackEnemyResponse::of(attack_enemy(input))
    }
}
