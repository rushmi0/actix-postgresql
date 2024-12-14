use actix_web::web;

mod index;
mod hello;
mod save;

use index::index_service;
use hello::hello_service;
use save::save_service;

pub fn service_hub(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(index_service)
            .service(hello_service)
            .service(save_service)
    );
}
