use actix_http::ResponseBuilder;
use actix_web::guard;
use actix_web::middleware;
use actix_web::{web, http, App, HttpServer};

mod db {
    include!("db.rs");
}

mod config {
    include!("config.rs");
}

pub mod routes;

pub mod models;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let pool = db::db_config();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(config::config)
            .default_service(web::route().guard(guard::Not(guard::Get())).to(
                || {
                    ResponseBuilder::new(http::StatusCode::from_u16(200).unwrap())
                        .set_header(
                            http::header::CONTENT_TYPE,
                            "application/json; charset=utf-8",
                        )
                        .body("{\"code\":404, \"message\":\"request not found\"}")
                }, 
            ))
        //.wrap(ErrorHandlers::new().handler(http::StatusCode::from_u16(404).unwrap(), render_500))
        //.service(web::resource("/index").route(web::get().to(person)))
    })
    .bind("127.0.0.1:8080")?
    .workers(4)
    .run()
    .await
}
