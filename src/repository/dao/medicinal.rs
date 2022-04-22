use crate::repository::{DBError, Dao, POOL};
use crate::util::serde_format::{i32_bool, naive_datetime};
use app_macro::Dao;
use async_trait::async_trait;
use chrono::{Date, DateTime, Local, NaiveDateTime};
use rbatis::{crud::CRUD, crud_table, wrapper::Wrapper};
use serde::Serialize;

#[crud_table(table_name: "medicinal")]
#[derive(Debug, Clone, Dao)]
pub struct Medicinal {
    pub id: u32,
    pub category_id: u32,
    pub user_id: String,
    pub name: String,
    pub batch_info: Option<String>,
    pub spec: Option<String>,
    pub count: Option<String>,
    // pub validate: Date<Local>,
    #[serde(serialize_with = "i32_bool::serialize")]
    pub is_deleted: i32,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub update_at: NaiveDateTime,
    pub notify_at: Option<u64>,
}
