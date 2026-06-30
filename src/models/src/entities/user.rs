use serde::{Deserialize, Serialize};
use crate::base::BaseModel;
use crate::entities::attributes::AttributesMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModel {
    pub user_id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub function: String,
    pub email: String,
    pub attributes: Option<AttributesMap>,
    pub metadata: BaseModel,
}

impl Default for UserModel {
    fn default() -> Self {
        Self {
            user_id: Default::default(),
            username: Default::default(),
            first_name: Default::default(),
            last_name: Default::default(),
            function: Default::default(),
            email: Default::default(),
            attributes: Default::default(),
            metadata: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserCreateModel {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub function: String,
    pub email: String,
    pub attributes: Option<AttributesMap>
}

impl Into<UserModel> for UserCreateModel {
    fn into(self) -> UserModel {
        UserModel {
            user_id: Default::default(),
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            function: self.function,
            email: self.email,
            attributes: self.attributes,
            metadata: BaseModel::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserUpdateModel {
    pub user_id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub function: String,
    pub email: String,
    pub attributes: Option<AttributesMap>,
}

impl Into<UserModel> for UserUpdateModel {
    fn into(self) -> UserModel {
        UserModel {
            user_id: Default::default(),
            username: String::new(),
            first_name: self.first_name,
            last_name: self.last_name,
            function: self.function,
            email: self.email,
            attributes: self.attributes,
            metadata: BaseModel::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserPagingResult {
    pub page_size: Option<u64>,
    pub page_index: Option<u64>,
    pub total_count: Option<u64>,
    pub users: Vec<UserModel>
}