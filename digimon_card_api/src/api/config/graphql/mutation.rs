use juniper::FieldResult;

use crate::api::config::graphql::schema::MutationRoot;
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
}