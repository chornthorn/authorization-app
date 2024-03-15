mod users;
mod roles;
mod permissions;
mod scopes;
mod resources;
mod policies;

use actix_web::web;

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(users::config)
            .configure(roles::config)
            .configure(permissions::config)
            .configure(scopes::config)
            .configure(resources::config)
            .configure(policies::config)
    );
}