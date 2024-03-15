use actix_web::{get, HttpResponse, post, Responder, web};
 pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/permissions")
            .service(index)
            .service(create)
            .service(get_by_id)
            .service(update)
            .service(delete),
    );
}

#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Get all permissions")
}

#[post("")]
async fn create() -> impl Responder {
    HttpResponse::Ok().json("Create a permission")
}

#[get("/{id}")]
async fn get_by_id() -> impl Responder {
    HttpResponse::Ok().json("Get permission by id")
}

#[post("/{id}")]
async fn update() -> impl Responder {
    HttpResponse::Ok().json("Update permission")
}

#[post("/{id}")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().json("Delete permission")
}