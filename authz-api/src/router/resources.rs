use actix_web::{get, HttpResponse, post, Responder, web};
 pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/resources")
            .service(index)
            .service(create)
            .service(get_by_id)
            .service(update)
            .service(delete),
    );
}

#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Get all resources")
}

#[post("")]
async fn create() -> impl Responder {
    HttpResponse::Ok().json("Create a resource")
}

#[get("/{id}")]
async fn get_by_id() -> impl Responder {
    HttpResponse::Ok().json("Get resource by id")
}

#[post("/{id}")]
async fn update() -> impl Responder {
    HttpResponse::Ok().json("Update resource")
}

#[post("/{id}")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().json("Delete resource")
}