use juniper::{EmptySubscription, RootNode};

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
