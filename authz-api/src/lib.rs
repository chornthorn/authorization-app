use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use sqlx::mysql::MySqlPoolOptions;
use authz_core::app_config::AppConfig;

#[actix_web::main]
pub async fn main() -> std::io::Result<()>{
    // load environment variables
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // log configuration
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // create a database pool
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1)
        }
    };

    let app_config = AppConfig::new("Todo Backend", pool);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(web::Data::new(AppConfig {
                name: app_config.name,
                pool: app_config.pool.clone(),
            }))
            .service(index)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json({
        let mut response = std::collections::HashMap::new();
        response.insert("message", "Welcome to the Todo Backend Api");
        response
    })
}
