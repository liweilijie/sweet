use crate::repository::dao;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    email: Option<String>,
    phone: Option<String>,
    created_at: NaiveDateTime,
    last_logined_at: NaiveDateTime,
}

impl From<dao::User> for User {
    fn from(d: dao::User) -> Self {
        Self {
            id: d.id,
            username: d.username,
            email: d.email,
            phone: d.phone,
            created_at: d.created_at,
            last_logined_at: d.last_logined_at,
        }
    }
}
