use crate::error::Error;
use crate::repository::{DBError, Dao, POOL};
use crate::util::serde_format::{i32_bool, naive_datetime};
use app_macro::Dao;
use async_trait::async_trait;
use rbatis::{crud::CRUD, crud_table, wrapper::Wrapper};
use serde::Serialize;

#[crud_table(table_name: "category")]
#[derive(Debug, Clone, Dao)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub user_id: String,
}

impl Category {
    pub async fn find_categories(user_id: &str) -> Result<Vec<Category>, Error> {
        let w = POOL.new_wrapper().eq("user_id", user_id);
        Ok(Category::find_list(w).await?)
    }
}
