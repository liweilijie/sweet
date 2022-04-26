pub mod dao;
pub mod dto;
pub mod vo;

use once_cell::sync::Lazy;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

const DATABASE_URL: &str = "DATABASE_URL";
pub static POOL: Lazy<DatabaseConnection> = Lazy::new(|| {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(10))
        .sqlx_logging(true);

    let db = Database::connect(opt).await.unwrap();
    db
});
