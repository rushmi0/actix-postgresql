use actix_web::{post, web, HttpResponse, Responder};
use log::info;
use crate::model::Event;
use crate::storage::statement::{StoredService, StoredServiceImpl};

#[post("/save")]
async fn save_service(
    event: web::Json<Event>,
    service: web::Data<StoredServiceImpl>,
) -> impl Responder {
    let event = event.into_inner();
    info!("{:?}", &event);

    match service.save(event).await {
        Some(true) => HttpResponse::Ok().body("Event saved successfully."),
        Some(false) => HttpResponse::BadRequest().body("Failed to save event (possibly a duplicate)."),
        None => HttpResponse::InternalServerError().body("Unexpected error while saving event."),
    }
}

/*
#[post("/save")]
async fn save_service(event: web::Json<Event>) -> impl Responder {
    let event = event.into_inner();
    info!("{:?}", &event);

    let service = StoredServiceImpl;
    match service.save(event).await {
        true => HttpResponse::Ok().body("Event saved successfully."),
        false => HttpResponse::InternalServerError().body("Failed to save event."),
    }
}
*/