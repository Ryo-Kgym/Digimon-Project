use actix_web::{App, guard, HttpResponse, HttpServer, Result, web};
use actix_web::web::Data;
use async_graphql::{EmptySubscription, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use query::MyQuery;
use mutation::MyMutation;

mod query;
mod mutation;
mod photo;


async fn index(schema: Data<Schema<MyQuery, MyMutation, EmptySubscription>>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(MyQuery, MyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
