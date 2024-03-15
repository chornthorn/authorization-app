use actix_web::{get, HttpResponse, post, Responder, web};
 pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(index)
            .service(create_user)
            .service(get_user_by_id)
            .service(update_user)
            .service(delete_user),
    );
}

#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("Get all users")
}

#[post("")]
async fn create_user() -> impl Responder {
    HttpResponse::Ok().json("Create a user")
}

#[get("/{id}")]
async fn get_user_by_id() -> impl Responder {
    HttpResponse::Ok().json("Get user by id")
}

#[post("/{id}")]
async fn update_user() -> impl Responder {
    HttpResponse::Ok().json("Update user")
}

#[post("/{id}")]
async fn delete_user() -> impl Responder {
    HttpResponse::Ok().json("Delete user")
}