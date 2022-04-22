pub mod dao;
pub mod dto;
pub mod vo;

mod traits;
pub use traits::Dao;

use once_cell::sync::Lazy;
use rbatis::logic_delete::RbatisLogicDeletePlugin;
use rbatis::{core::Error, rbatis::Rbatis};
use std::env;

pub type DBPool = Rbatis;
pub type DBError = Error;

const DATABASE_URL: &str = "DATABASE_URL";
pub static POOL: Lazy<DBPool> = Lazy::new(|| {
    let mut rb = Rbatis::new();
    rb.set_logic_plugin(RbatisLogicDeletePlugin::new_opt("is_deleted", 1, 0));
    rb
});

pub async fn init_db() {
    let database_url =
        env::var(DATABASE_URL).expect("environment variable DATABASE_URL must be set");
    POOL.link(&database_url)
        .await
        .expect("connect to database failed.");
}
