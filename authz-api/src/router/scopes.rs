use actix_web::{get, HttpResponse, post, Responder, web};
 pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/scopes")
            .service(index)
            .service(create)
            .service(get_by_id)
            .service(update)
            .service(delete),
    );
}

#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Get all scopes")
}

#[post("")]
async fn create() -> impl Responder {
    HttpResponse::Ok().json("Create a scope")
}

#[get("/{id}")]
async fn get_by_id() -> impl Responder {
    HttpResponse::Ok().json("Get scope by id")
}

#[post("/{id}")]
async fn update() -> impl Responder {
    HttpResponse::Ok().json("Update scope")
}

#[post("/{id}")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().json("Delete scope")
}