use actix_web::web;

use super::handlers;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/user")
        .service(handlers::user_handler::create_user)
        .service(handlers::user_handler::get_user_by_id)
    );
}