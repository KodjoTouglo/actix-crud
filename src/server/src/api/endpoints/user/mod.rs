use actix_web::{post, put, delete, get, web, Responder};

use models::entities::user::{UserCreateModel, UserModel, UserUpdateModel};
use models::PagingParams;
use services::session::session::BackendSession;
use crate::api::api_services::user_api::UserApi;

#[post("/create")]
pub async fn create_user(
    user: web::Json<UserCreateModel>,
    session: web::Data<BackendSession>,
) -> impl Responder {
    let  create_user: UserModel = user.0.into();
    log::info!("Creating user");
    UserApi::create_user(&session, create_user).await
}

#[put("/update/{user_id}")]
pub async fn update_user(
    params: web::Path<String>,
    user: web::Json<UserUpdateModel>,
    session: web::Data<BackendSession>
) -> impl Responder {
    let user_id = params.into_inner();
    let mut user_model: UserModel = user.0.into();
    log::info!("Updating user {}", &user_id);
    user_model.user_id = user_id;
    UserApi::update_user(&session, user_model).await
}

#[delete("/delete/{user_id}")]
pub async fn delete_user(
    params: web::Path<String>,
    session: web::Data<BackendSession>,
) -> impl Responder {
    let user_id = params.into_inner();
    log::info!("Deleting user {}", &user_id);
    UserApi::delete_user(&session, &user_id).await
}

#[get("/{user_id}/load")]
pub async fn load_user(
    params: web::Path<String>,
    session: web::Data<BackendSession>,
) -> impl Responder {
    let user_id = params.into_inner();
    log::info!("Loading user {}", &user_id);
    UserApi::load_user_by_id(&session, &user_id).await
}

#[get("/count-users")]
pub async fn count_users(
    session: web::Data<BackendSession>
) -> impl Responder {
    log::info!("Counting users");
    UserApi::count_users(&session).await
}

#[get("/load-all")]
pub async fn load_users(
    paging: web::Query<PagingParams>,
    session: web::Data<BackendSession>,
) -> impl Responder {
    log::info!("Loading users");
    UserApi::load_users(&session, &paging.0).await
}