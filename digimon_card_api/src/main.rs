use std::{io, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    App, get, HttpResponse,
    HttpServer,
    middleware, Responder, route, web::{self, Data},
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use crate::api::config::graphql::schema::{create_schema, Schema};

mod api;

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let schema = Arc::new(create_schema());

    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
        .workers(2)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use crate::create_schema;

    #[test]
    fn main() {
        let result = create_schema().as_schema_language();

        let expected =
"\
type QueryRoot {
  dummy(id: String!): String!
}

type MutationRoot {
  \"My Digimon attacks the enemy.\"
  attackEnemy(request: AttackEnemyRequest!): HitPoint!
  \"My Digimon is attacked by the enemy.\"
  beAttacked(request: BeAttackedRequest!): HitPoint!
}

input BeAttackedRequest {
  myHitPointValue: Int!
  enemyAttackValue: Int!
}

type HitPoint {
  value: Int!
}

input AttackEnemyRequest {
  myAttackValue: Int!
  enemyHitPointValue: Int!
}

schema {
  query: QueryRoot
  mutation: MutationRoot
}
";
        assert_eq!(result, expected);
    }
}