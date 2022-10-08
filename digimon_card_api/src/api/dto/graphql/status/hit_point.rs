use juniper::GraphQLObject;

#[derive(GraphQLObject)]
#[derive(Debug, PartialEq)]
pub struct HitPoint {
    pub value: i32,
}