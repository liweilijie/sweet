use crate::error::Error;
use crate::repository::{DBError, Dao, POOL};
use crate::util::serde_format::{i32_bool, naive_datetime};
use app_macro::Dao;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use rbatis::{crud::CRUD, crud_table, wrapper::Wrapper};
use serde::Serialize;

#[crud_table(table_name: "users")]
#[derive(Debug, Clone, Dao)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[serde(serialize_with = "i32_bool::serialize")]
    pub is_actived: i32,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub last_logined_at: NaiveDateTime,
    #[serde(serialize_with = "naive_datetime::serialize")]
    pub created_at: NaiveDateTime,
}

impl User {
    pub async fn find_by_username(username: &str) -> Result<Self, Error> {
        let w = POOL.new_wrapper().eq("username", username);
        Ok(Self::find_one(w).await?)
    }

    pub async fn find_by_username_or_email(username_or_email: &str) -> Result<Self, Error> {
        let w = POOL
            .new_wrapper()
            .eq("username", username_or_email)
            .or()
            .eq("email", username_or_email);
        Ok(Self::find_one(w).await?)
    }
}
