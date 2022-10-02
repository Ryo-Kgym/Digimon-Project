use actix_web::{App, guard, HttpResponse, HttpServer, Result, web};
use actix_web::web::Data;
use async_graphql::{EmptySubscription, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use mutation::MyMutation;
use query::MyQuery;

mod query;
mod mutation;
mod photo;


async fn post_index(schema: Data<Schema<MyQuery, MyMutation, EmptySubscription>>,
                    req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn get_index() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/")
        .subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(MyQuery, MyMutation, EmptySubscription)
        .finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(post_index))
            .service(web::resource("/").guard(guard::Get()).to(get_index))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
