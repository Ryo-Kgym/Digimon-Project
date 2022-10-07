use juniper::FieldResult;
use crate::api::config::graphql::schema::QueryRoot;

use crate::api::dto::graphql::human::{Episode, Human};


#[juniper::graphql_object]
impl QueryRoot {
    fn human(_id: String) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
}