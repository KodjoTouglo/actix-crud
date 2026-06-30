use std::sync::Arc;
use async_trait::async_trait;
use shaku::{Component, Interface};
use models::entities::user::{UserModel, UserPagingResult};
use store::providers::interfaces::user::IUserProvider;

#[async_trait]
pub trait IUserService: Interface {
    async fn create_user(&self, user: &UserModel) -> Result<(), String>;

    async fn update_user(&self, user: &UserModel) -> Result<(), String>;

    async fn delete_user(&self, user_id: &str) -> Result<(), String>;

    async fn load_user_by_id(&self, user_id: &str) -> Result<Option<UserModel>, String>;

    async fn load_user_by_ids(&self, user_ids: &[&str]) -> Result<Vec<UserModel>, String>;
    
    async fn load_user_by_username(&self,username: &str) -> Result<Option<UserModel>, String>;
    
    async fn load_user_by_email(&self, email: &str) -> Result<Option<UserModel>, String>;

    async fn user_exists_by_username(&self, username: &str) ->  Result<bool, String>;

    async fn user_exists_by_email(&self, email: &str) -> Result<bool, String>;

    async fn load_users_paging(&self, page: &Option<u64>, size: &Option<u64>) -> Result<UserPagingResult, String>;

    async fn count_users(&self) -> Result<u64, String>;
}

#[allow(dead_code)]
#[derive(Component)]
#[shaku(interface = IUserService)]
pub struct UserService {
    #[shaku(inject)]
    user_repo: Arc<dyn IUserProvider>,
}

#[async_trait]
impl IUserService for UserService {
    async fn create_user(&self, user: &UserModel) -> Result<(), String> {
        self.user_repo.create_user(&user).await
    }

    async fn update_user(&self, user: &UserModel) -> Result<(), String> {
        self.user_repo.update_user(&user).await
    }

    async fn delete_user(&self, user_id: &str) -> Result<(), String> {
        self.user_repo.delete_user(&user_id).await
    }

    async fn load_user_by_id(&self, user_id: &str) -> Result<Option<UserModel>, String> {
        self.user_repo.load_user_by_id(&user_id).await
    }

    async fn load_user_by_ids(&self, user_ids: &[&str]) -> Result<Vec<UserModel>, String> {
        self.user_repo.load_user_by_ids(&user_ids).await
    }

    async fn load_user_by_username(&self, username: &str) -> Result<Option<UserModel>, String> {
        self.user_repo.load_user_by_username(&username).await
    }

    async fn load_user_by_email(&self, email: &str) -> Result<Option<UserModel>, String> {
        self.user_repo.load_user_by_email(&email).await
    }

    async fn user_exists_by_username(&self, username: &str) -> Result<bool, String> {
        self.user_repo.user_exists_by_username(&username).await
    }

    async fn user_exists_by_email(&self, email: &str) -> Result<bool, String> {
        self.user_repo.user_exists_by_email(&email).await
    }


    async fn load_users_paging(&self, page_index: &Option<u64>, page_size: &Option<u64>) -> Result<UserPagingResult, String> {
        self.user_repo.load_users_paging(&page_index, &page_size).await
    }

    async fn count_users(&self) -> Result<u64, String> {
        self.user_repo.count_users().await
    }
}
