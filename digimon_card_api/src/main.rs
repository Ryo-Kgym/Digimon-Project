use actix_web::{App, guard, HttpServer, web};
use actix_web::web::Data;

use api::config::graphql::{get_index, post_index, schema};

mod api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Graph iQL is here: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema().clone()))
            .service(web::resource("/")
                .guard(guard::Post()).to(post_index))
            .service(web::resource("/")
                .guard(guard::Get()).to(get_index))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
