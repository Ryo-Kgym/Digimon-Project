use juniper::FieldResult;

use digimon_card_core::core::domain::use_case::attack::attack_enemy::attack_enemy;

use crate::api::config::graphql::schema::MutationRoot;
use crate::api::dto::graphql::attack_enemy::{
    AttackEnemyRequest,
    AttackEnemyResponse,
};
use crate::api::dto::graphql::human::{Human, NewHuman};

#[juniper::graphql_object]
impl MutationRoot {
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }

    #[graphql(description = "My Digimon attacks the enemy.")]
    fn attack_enemy(request: AttackEnemyRequest) -> AttackEnemyResponse {
        let input = request.to_attack_enemy_input();
        AttackEnemyResponse::of(attack_enemy(input))
    }
}
