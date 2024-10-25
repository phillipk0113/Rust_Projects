use actix_web::{HttpResponse, Responder, web};
use crate::handlers::{create_note, get_notes, get_note_by_id, update_note, delete_note};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(health_check)));
    cfg.service(
        web::resource("/notes")
        .route(web::post().to(create_note))
        .route(web::get().to(get_notes))
    );
    cfg.service(
        web::resource("/notes/{id}")
        .route(web::get().to(get_note_by_id))
        .route(web::put().to(update_note))
        .route(web::delete().to(delete_note))
    );
}
