use juniper::FieldResult;
use crate::api::config::graphql::schema::QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn dummy(_id: String) -> FieldResult<String> {
        Ok(String::from("dummy"))
    }
}