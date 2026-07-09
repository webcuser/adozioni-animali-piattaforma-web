use actix_web::web;
use crate::auth::{register_user, login_user};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/register")
            .route(web::post().to(register_user))
    );
    cfg.service(
        web::resource("/login")
            .route(web::post().to(login_user))
    );
}
