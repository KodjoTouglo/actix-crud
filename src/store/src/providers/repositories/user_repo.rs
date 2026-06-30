use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Object;
use postgres_types::ToSql;
use serde_json::json;
use shaku::Component;
use tokio_postgres::Row;

use models::base::BaseModel;
use models::{
    entities::attributes::AttributesMap,
    entities::user::{UserModel, UserPagingResult},
};

use crate::providers::core::builder::{
    DeleteQueryBuilder, InsertQueryBuilder, SelectCountQueryBuilder, SelectQueryBuilder,
    SqlCriteriaBuilder, UpdateQueryBuilder,
};
use crate::providers::tables::user_table;
use crate::providers::{
    client::postgres_client::IDataBaseManager, interfaces::user::IUserProvider,
};

#[allow(dead_code)]
#[derive(Component)]
#[shaku(interface = IUserProvider)]
pub struct UserProvider {
    #[shaku(inject)]
    pub database_manager: Arc<dyn IDataBaseManager>,
}

impl UserProvider {
    fn parse_user(row: &Row) -> UserModel {
        let attributes: Option<AttributesMap> = serde_json::from_value::<AttributesMap>(
            row.get::<&str, serde_json::Value>("attributes"),
        )
        .map_or_else(|_| None, |p| Some(p));

        UserModel {
            user_id: row.get("user_id"),
            username: row.get("username"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            function: row.get("function"),
            email: row.get("email"),
            attributes,
            metadata: BaseModel {
                created_by: row.get("created_by"),
                created_at: row.get("created_at"),
                updated_by: row.get("updated_by"),
                updated_at: row.get("updated_at"),
                version: row.get("version"),
            },
        }
    }

    pub async fn load_users_by_query(
        client: &Object,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<UserModel>, String> {
        let load_users_stmt = client.prepare_cached(&query).await.unwrap();
        let result = client.query(&load_users_stmt, &params).await;
        match result {
            Ok(rows) => Ok(rows
                .iter()
                .map(|row| UserProvider::parse_user(&row))
                .collect()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn load_user_by_query(
        client: &Object,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<UserModel>, String> {
        let load_users_stmt = client.prepare_cached(&query).await.unwrap();
        let result = client.query_opt(&load_users_stmt, &params).await;
        match result {
            Ok(Some(row)) => Ok(Some(UserProvider::parse_user(&row))),
            Ok(_) => Ok(None),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn user_exist_by_criteria(
        client: &Object,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<bool, String> {
        let load_user_stmt = client.prepare_cached(&query).await.unwrap();
        let result = client.query_one(&load_user_stmt, &params).await;
        match result {
            Ok(row) => Ok(row.get::<usize, i64>(0) > 0),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[async_trait]
impl IUserProvider for UserProvider {
    async fn create_user(&self, user: &UserModel) -> Result<(), String> {
        let client = self.database_manager.connection().await;
        if let Err(err) = client {
            return Err(err);
        }
        let create_user_sql = InsertQueryBuilder::new()
            .table_name(user_table::USERS_TABLE.table_name.clone())
            .columns(user_table::USERS_TABLE.insert_columns.clone())
            .resolve_conflict(false)
            .sql_query()
            .unwrap();
        let mut client = client.unwrap();
        let transaction = client.transaction().await;
        match transaction {
            Ok(trx) => {
                let response = trx
                    .execute(
                        &create_user_sql,
                        &[
                            &user.user_id,
                            &user.username,
                            &user.first_name,
                            &user.last_name,
                            &user.function,
                            &user.email,
                            &json!(user.attributes),
                            &user.metadata.created_by,
                            &user.metadata.created_at,
                            &user.metadata.version,
                        ],
                    )
                    .await;
                match response {
                    Err(err) => return Err(err.to_string()),
                    _ => {}
                }

                match trx.commit().await {
                    Err(err) => return Err(err.to_string()),
                    Ok(_) => Ok(()),
                }
            }
            Err(err) => return Err(err.to_string()),
        }
    }

    async fn update_user(&self, user: &UserModel) -> Result<(), String> {
        let client = self.database_manager.connection().await;
        if let Err(err) = client {
            return Err(err);
        }
        let update_user_sql = UpdateQueryBuilder::new()
            .table_name(user_table::USERS_TABLE.table_name.clone())
            .columns(user_table::USERS_TABLE.update_columns.clone())
            .where_clauses(vec![SqlCriteriaBuilder::is_equals("user_id".to_string())])
            .manage_version(true)
            .sql_query()
            .unwrap();

        let client = client.unwrap();
        let update_user_stmt = client.prepare_cached(&update_user_sql).await.unwrap();

        let response = client
            .execute(
                &update_user_stmt,
                &[
                    &user.username,
                    &user.first_name,
                    &user.last_name,
                    &user.function,
                    &user.email,
                    &json!(user.attributes),
                    &user.metadata.updated_by,
                    &user.metadata.updated_at,
                    &user.user_id,
                ],
            )
            .await;
        match response {
            Err(err) => Err(err.to_string()),
            Ok(response) => {
                return if response == 1 {
                    Ok(())
                } else {
                    Err("Failed to update user".to_string())
                }
            }
        }
    }

    async fn delete_user(&self, user_id: &str) -> Result<(), String> {
        let client = self.database_manager.connection().await;
        if let Err(err) = client {
            return Err(err);
        }
        let client = client.unwrap();

        let delete_user_sql = DeleteQueryBuilder::new()
            .table_name(user_table::USERS_TABLE.table_name.clone())
            .where_clauses(vec![SqlCriteriaBuilder::is_equals("user_id".to_string())])
            .sql_query()
            .unwrap();

        let delete_user_stmt = client.prepare_cached(&delete_user_sql).await.unwrap();
        let mut client = client;
        let transaction = client.transaction().await;

        match transaction {
            Ok(trx) => {
                if let Err(err) = trx.execute(&delete_user_stmt, &[&user_id]).await {
                    return Err(err.to_string());
                }
                if let Err(err) = trx.commit().await {
                    return Err(err.to_string());
                };
                return Ok(());
            }
            Err(err) => Err(err.to_string()),
        }
    }

    async fn load_user_by_id(&self, user_id: &str) -> Result<Option<UserModel>, String> {
        let client = self.database_manager.connection().await;
        if let Err(err) = client {
            return Err(err);
        }
        let load_user_sql = SelectQueryBuilder::new()
            .table_name(user_table::USERS_TABLE.table_name.clone())
            .where_clauses(vec![SqlCriteriaBuilder::is_equals("user_id".to_string())])
            .sql_query()
            .unwrap();
        let client = client.unwrap();
        let params: Vec<&(dyn ToSql + Sync)> = vec![&user_id];
        UserProvider::load_user_by_query(&client, &load_user_sql, &params).await
    }

    async fn load_user_by_ids(&self, user_ids: &[&str]) -> Result<Vec<UserModel>, String> {
        let client = self.database_manager.connection().await;
        if let Err(err) = client {
            return Err(err);
        }
        let load_users_sql = SelectQueryBuilder::new()
            .table_name(user_table::USERS_TABLE.table_name.clone())
            .where_clauses(vec![SqlCriteriaBuilder::is_in(
                "user_id".to_string(),
                user_ids.len(),
            )])
            .sql_query()
            .unwrap();

        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for user_id in user_ids.iter() {
            params.push(user_id)
        }
        UserProvider::load_users_by_query(&client.unwrap(), &load_users_sql, &params).await
    }

    async fn load_user_by_username(&self, username: &str) -> Result<Option<UserModel>, String> {
        let client = self.database_manager.connection().await;
        if let Err(err) = client {
            return Err(err);
        }
        let load_user_by_username_sql = SelectQueryBuilder::new()
            .table_name(user_table::USERS_TABLE.table_name.clone())
            .where_clauses(vec![SqlCriteriaBuilder::is_equals("email".to_string())])
            .sql_query()
            .unwrap();
        let client = client.unwrap();
        let params: Vec<&(dyn ToSql + Sync)> = vec![&username];
        UserProvider::load_user_by_query(&client, &load_user_by_username_sql, &params).await
    }

    async fn load_user_by_email(&self, email: &str) -> Result<Option<UserModel>, String> {
        let client = self.database_manager.connection().await;
        if let Err(err) = client {
            return Err(err);
        }
        let load_user_by_email_sql = SelectQueryBuilder::new()
            .table_name(user_table::USERS_TABLE.table_name.clone())
            .where_clauses(vec![SqlCriteriaBuilder::is_equals("email".to_string())])
            .sql_query()
            .unwrap();
        let client = client.unwrap();
        let params: Vec<&(dyn ToSql + Sync)> = vec![&email];
        UserProvider::load_user_by_query(&client, &load_user_by_email_sql, &params).await
    }

    async fn user_exists_by_username(&self, username: &str) -> Result<bool, String> {
        let client = self.database_manager.connection().await;
        if let Err(err) = client {
            return Err(err);
        }
        let load_user_sql = SelectCountQueryBuilder::new()
            .table_name(user_table::USERS_TABLE.table_name.clone())
            .where_clauses(vec![SqlCriteriaBuilder::is_equals("username".to_string())])
            .sql_query()
            .unwrap();
        let params: Vec<&(dyn ToSql + Sync)> = vec![&username];
        UserProvider::user_exist_by_criteria(&client.unwrap(), &load_user_sql, &params).await
    }

    async fn user_exists_by_email(&self, email: &str) -> Result<bool, String> {
        let client = self.database_manager.connection().await;
        if let Err(err) = client {
            return Err(err);
        }
        let load_user_sql = SelectCountQueryBuilder::new()
            .table_name(user_table::USERS_TABLE.table_name.clone())
            .where_clauses(vec![SqlCriteriaBuilder::is_equals("email".to_string())])
            .sql_query()
            .unwrap();
        let params: Vec<&(dyn ToSql + Sync)> = vec![&email];
        UserProvider::user_exist_by_criteria(&client.unwrap(), &load_user_sql, &params).await
    }

    async fn count_users(&self) -> Result<u64, String> {
        let client = self.database_manager.connection().await;
        if let Err(err) = client {
            return Err(err);
        }
        let count_users_sql = SelectCountQueryBuilder::new()
            .table_name(user_table::USERS_TABLE.table_name.clone())
            .sql_query()
            .unwrap();
        let client = client.unwrap();
        let count_users_stmt = client.prepare_cached(&count_users_sql).await.unwrap();
        let result = client.query_one(&count_users_stmt, &[]).await;
        match result {
            Ok(row) => Ok(row.get::<usize, i64>(0) as u64),
            Err(err) => Err(err.to_string()),
        }
    }

    async fn load_users_paging(
        &self,
        page_index: &Option<u64>,
        page_size: &Option<u64>,
    ) -> Result<UserPagingResult, String> {
        let client = self.database_manager.connection().await;
        if let Err(err) = client {
            return Err(err);
        }

        let client = client.unwrap();
        let count_user_stmt = client
            .prepare_cached(&user_table::SELECT_COUNT_USERS)
            .await
            .unwrap();
        let count_result = client.query_one(&count_user_stmt, &[]).await;
        if let Err(err) = count_result {
            return Err(err.to_string());
        }
        let total_users = count_result.unwrap().get::<usize, i64>(0);

        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let load_user_sql;
        let page_offset;
        let page_size_v;

        if page_index.is_none() || page_size.is_none() {
            load_user_sql = user_table::SELECT_USERS.clone();
        } else {
            load_user_sql = user_table::SELECT_USERS_PAGING.clone();
            page_offset = (page_index.unwrap() * page_size.unwrap()) as i64;
            page_size_v = page_size.unwrap() as i64;
            params.push(&page_offset);
            params.push(&page_size_v);
        }

        let load_users_stmt = client.prepare_cached(&load_user_sql).await.unwrap();
        let result = client.query(&load_users_stmt, &params).await;
        match result {
            Ok(rows) => {
                let users = rows
                    .iter()
                    .map(|row| UserProvider::parse_user(&row))
                    .collect();
                return Ok(UserPagingResult {
                    page_size: page_size.clone(),
                    page_index: page_index.clone(),
                    total_count: Some(total_users as u64),
                    users,
                });
            }
            Err(err) => Err(err.to_string()),
        }
    }
}
