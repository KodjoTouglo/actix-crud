use async_trait::async_trait;
use models::entities::user::{UserModel, UserPagingResult};
use shaku::Interface;

#[async_trait]
pub trait IUserProvider: Interface {
    async fn create_user(&self, user: &UserModel) -> Result<(), String>;

    async fn update_user(&self, user: &UserModel) -> Result<(), String>;

    async fn delete_user(&self, user_id: &str) -> Result<(), String>;

    async fn load_user_by_id(&self, user_id: &str) -> Result<Option<UserModel>, String>;

    async fn load_user_by_ids(&self, user_ids: &[&str]) -> Result<Vec<UserModel>, String>;

    async fn load_user_by_username(&self, username: &str) -> Result<Option<UserModel>, String>;

    async fn load_user_by_email(&self, email: &str) -> Result<Option<UserModel>, String>;

    async fn user_exists_by_username(&self, username: &str) -> Result<bool, String>;

    async fn user_exists_by_email(&self, email: &str) -> Result<bool, String>;

    async fn count_users(&self) -> Result<u64, String>;

    async fn load_users_paging(
        &self,
        page_index: &Option<u64>,
        page_size: &Option<u64>,
    ) -> Result<UserPagingResult, String>;
}
