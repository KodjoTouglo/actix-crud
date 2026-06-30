use commons::ApiResult;
use models::base::BaseModel;
use models::entities::user::{UserModel, UserPagingResult};
use models::PagingParams;
use services::session::session::BackendSession;

#[allow(dead_code)]
pub struct UserApi;

#[allow(unreachable_patterns)]
impl UserApi {
    pub async fn create_user(
        session: &BackendSession,
        user: UserModel,
    ) -> ApiResult<UserModel> {
        let existing_username = session
            .services()
            .user_service()
            .user_exists_by_username(&user.username)
            .await;
        if let Ok(response) = existing_username {
            if response {
                log::error!("username: {} already exists", &user.username);
                return ApiResult::from_error(409, "409", "username already exists");
            }
        }

        let existing_email = session
            .services()
            .user_service()
            .user_exists_by_email(&user.email)
            .await;
        if let Ok(response) = existing_email {
            if response {
                log::error!("email: {} already exists", &user.email);
                return ApiResult::from_error(409, "409", "email already exists");
            }
        }

        let mut create_user = user;
        create_user.user_id = uuid::Uuid::new_v4().to_string();
        create_user.metadata = BaseModel::from_creator("michel".to_string());
        let created_user = session
            .services()
            .user_service()
            .create_user(&create_user)
            .await;
        match created_user {
            _ => ApiResult::Data(create_user),
            Err(err) => ApiResult::from_error(500, "500", err.as_str()),
        }
    }

    pub async fn update_user(session: &BackendSession, user: UserModel) -> ApiResult {
        let existing_user = session
            .services()
            .user_service()
            .load_user_by_id(&user.user_id)
            .await;
        match &existing_user {
            Ok(checked) => {
                if checked.is_none() {
                    log::error!("User: {} not found", &user.user_id);
                    return ApiResult::from_error(404, "404", "user not found");
                }
            }
            Err(err) => {
                log::error!("Failed to load user: {}", &user.user_id);
                return ApiResult::from_error(500, "500", err.as_str());
            }
        }

        let mut user_model = existing_user.unwrap().unwrap();
        user_model.username = user.username;
        user_model.first_name = user.first_name;
        user_model.last_name = user.last_name;
        user_model.function = user.function;
        user_model.email = user.email;
        user_model.attributes = user.attributes;
        user_model.metadata = BaseModel::from_updater("michel".to_owned());
        let updated_user = session.services().user_service().update_user(&user_model).await;
        match updated_user {
            Err(err) => ApiResult::from_error(500, "500", err.as_str()),
            _ => ApiResult::no_content(),
        }
    }

    pub async fn delete_user(session: &BackendSession, user_id: &str) -> ApiResult<()> {
        let response = session
            .services()
            .user_service()
            .delete_user(&user_id)
            .await;
        match response {
            Err(err) => ApiResult::from_error(500, "500", err.as_str()),
            _ => ApiResult::no_content(),
        }
    }

    pub async fn load_user_by_id(session: &BackendSession, user_id: &str) -> ApiResult<UserModel> {
        let loaded_user = session
            .services()
            .user_service()
            .load_user_by_id(&user_id)
            .await;
        match loaded_user {
            Ok(user) => ApiResult::<UserModel>::from_option(user),
            Err(err) => ApiResult::from_error(500, "500", &err),
        }
    }

    pub async fn load_users(
        session: &BackendSession,
        paging: &PagingParams,
    ) -> ApiResult<UserPagingResult> {
        let loaded_users = session
            .services()
            .user_service()
            .load_users_paging(&&paging.page_index, &paging.page_size)
            .await;
        match loaded_users {
            Ok(users) => {
                log::info!("[{}] users loaded", users.users.len());
                if users.users.is_empty() {
                    ApiResult::no_content()
                } else {
                    ApiResult::from_data(users)
                }
            }
            Err(err) => ApiResult::from_error(500, "500", &err),
        }
    }

    pub async fn count_users(session: &BackendSession) -> ApiResult<u64> {
        let response = session.services().user_service().count_users().await;
        match response {
            Ok(count) => ApiResult::from_data(count),
            Err(err) => ApiResult::from_error(500, "500", &err),
        }
    }
}
