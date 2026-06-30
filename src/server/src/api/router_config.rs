use super::endpoints::{health, user};
use actix_web::web;

pub fn register_apis(api_config: &mut web::ServiceConfig) {
    api_config.service(
        web::scope("/api/v1/actix-boilerplate")
            .service(web::scope("/monitoring").service(health::health_check))
            .service(
                web::scope("/user")
                    .service(user::create_user)
                    .service(user::load_user)
                    .service(user::update_user)
                    .service(user::load_users)
                    .service(user::count_users)
                    .service(user::delete_user),
            ),
    );
}
