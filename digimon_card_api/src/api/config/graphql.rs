// use actix_web::{HttpResponse, Result};
// use actix_web::web::Data;
// use async_graphql::{EmptySubscription, Schema};
// use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
// use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
//
// use crate::api::persistence::graphql::mutation::DigimonMutation;
// use crate::api::persistence::graphql::query::DigimonQuery;
//
// pub async fn post_index(schema: Data<Schema<DigimonQuery, DigimonMutation, EmptySubscription>>,
//                         req: GraphQLRequest) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }
//
// pub async fn get_index() -> Result<HttpResponse> {
//     let source = playground_source(GraphQLPlaygroundConfig::new("/")
//         .subscription_endpoint("/"));
//     Ok(HttpResponse::Ok()
//         .content_type("text/html; charset=utf-8")
//         .body(source))
// }
//
// pub fn schema() -> Schema<DigimonQuery, DigimonMutation, EmptySubscription> {
//     Schema::build(DigimonQuery, DigimonMutation, EmptySubscription)
//         .finish()
// }
//
