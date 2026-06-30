use serde::Deserialize;

pub mod entities;
pub mod base;

#[derive(Deserialize, Debug)]
pub struct PagingParams {
    pub page_index: Option<u64>,
    pub page_size: Option<u64>,
}
