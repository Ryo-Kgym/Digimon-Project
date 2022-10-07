use juniper::{EmptySubscription, FieldResult, RootNode};

use crate::api::dto::graphql::human::{Episode, Human, NewHuman};

pub struct MutationRoot;

pub struct QueryRoot;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot {},
        MutationRoot {},
        EmptySubscription::new(),
    )
}
