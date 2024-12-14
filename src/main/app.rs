use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use env_logger::{init_from_env, Env};

use crate::services::api::v1;
use crate::storage::initialize;
use crate::storage::statement::StoredServiceImpl;

pub async fn run() -> std::io::Result<()> {
    init_from_env(Env::default().default_filter_or("info"));
    initialize();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(StoredServiceImpl))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(cors_config())
            .configure(v1::service_hub)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn cors_config() -> Cors {
    Cors::default()
        .allow_any_origin()
        .send_wildcard()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}
