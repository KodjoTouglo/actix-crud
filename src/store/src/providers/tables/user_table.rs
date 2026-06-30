use crate::providers::tables::table::Table;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref USERS_TABLE: Table = Table {
        table_name: "USERS".to_owned(),
        insert_columns: vec![
            "user_id".to_owned(),
            "username".to_owned(),
            "first_name".to_owned(),
            "last_name".to_owned(),
            "function".to_owned(),
            "email".to_owned(),
            "attributes".to_owned(),
            "created_by".to_owned(),
            "created_at".to_owned(),
            "version".to_owned()
        ],
        update_columns: vec![
            "username".to_owned(),
            "first_name".to_owned(),
            "last_name".to_owned(),
            "function".to_owned(),
            "email".to_owned(),
            "attributes".to_owned(),
            "updated_by".to_owned(),
            "updated_at".to_owned()
        ]
    };
    pub static ref SELECT_COUNT_USERS: String = "SELECT COUNT(*) FROM USERS".to_owned();
    pub static ref SELECT_USERS_PAGING: String =
        "SELECT u.* FROM USERS u OFFSET $1 LIMIT $2".to_owned();
    pub static ref SELECT_USERS: String = "SELECT u.* FROM USERS u".to_owned();
}
