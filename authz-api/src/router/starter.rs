use actix_web::{get, HttpResponse, post, Responder, web};
 pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/starter")
            .service(index)
            .service(create)
            .service(get_by_id)
            .service(update)
            .service(delete),
    );
}

#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Get all starters")
}

#[post("")]
async fn create() -> impl Responder {
    HttpResponse::Ok().json("Create a starter")
}

#[get("/{id}")]
async fn get_by_id() -> impl Responder {
    HttpResponse::Ok().json("Get starter by id")
}

#[post("/{id}")]
async fn update() -> impl Responder {
    HttpResponse::Ok().json("Update starter")
}

#[post("/{id}")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().json("Delete starter")
}