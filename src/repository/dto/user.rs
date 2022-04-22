use crate::error::Result;
use crate::repository::{dao::User, vo, Dao, POOL};
use crate::util::util::now;
use bcrypt::{hash, verify};
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::{Page, PageRequest};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 1, max = 128))]
    pub username: String,
    #[validate(length(min = 1, max = 256))]
    pub password: String,
    #[validate(must_match(other = "password", message = "密码不匹配"))]
    pub repeat_password: String,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(phone)]
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub memo: Option<String>,
    pub sys_role: Option<String>,
    #[serde(default = "now")]
    pub last_logined_at: NaiveDateTime,
}

impl NewUser {
    pub async fn create(self) -> Result<User> {
        let id = Uuid::new_v4().to_string();
        let hashed_password = hash(&self.password, 4).unwrap();
        let dao = User {
            id: id.clone(),
            username: self.username,
            password: hashed_password,
            email: Some(self.email.unwrap_or("".to_string())),
            last_logined_at: now(),
            created_at: now(),
            is_actived: 1,
            phone: Some(self.phone.unwrap_or("".to_string())),
        };

        User::create_one(&dao).await?;
        Ok(dao)
    }
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct UpdateUser {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(phone)]
    pub phone: Option<String>,
}

impl UpdateUser {
    pub async fn save(self, dao: &mut User) -> Result<vo::User> {
        if self.email.is_none() && self.phone.is_none() {
            return Err(crate::error::Error::Custom("没有更新任何字段".to_string()));
        }
        dao.email = Some(self.email.unwrap_or("".to_string()));
        dao.phone = Some(self.phone.unwrap_or("".to_string()));
        let w = POOL.new_wrapper().eq("id", &dao.id);
        User::update_one(&dao, w).await?;
        Ok(dao.to_owned().into())
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginUser {
    #[validate(length(min = 1, max = 256))]
    pub username_or_email: String,
    #[validate(length(min = 1, max = 256))]
    pub password: String,
}

impl LoginUser {
    pub fn is_password_matched(&self, target: &str) -> bool {
        verify(self.password.clone(), target).unwrap()
    }

    pub async fn login(&self, dao: &User) -> Result<User> {
        let mut dao = dao.to_owned();
        dao.last_logined_at = now();
        let w = POOL.new_wrapper().eq("id", dao.id.clone());
        User::update_one(&dao, w).await?;
        Ok(dao)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct QueryUser {
    pub key: Option<String>,
    #[validate(range(min = 1))]
    page: Option<u64>,
    #[validate(range(min = 1))]
    limit: Option<u64>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}

impl QueryUser {
    pub async fn find_all(self) -> Result<Page<User>> {
        let page = self.page.unwrap_or(1);
        let limit = self.limit.unwrap_or(4);
        let req = PageRequest::new(page, limit);
        let sort_by = self.sort_by.unwrap_or("created_at".to_string());
        let sort_order = self.sort_order.unwrap_or("DESC".to_string());
        let mut w = POOL.new_wrapper();
        if let Some(key) = self.key {
            if key != "" {
                w = w.and().like("username", key);
            }
        }
        w = w.order_by(&sort_order.to_uppercase() == "ASC", &[&sort_by]);
        Ok(POOL.fetch_page_by_wrapper::<User>(w, &req).await?)
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ChangePassword {
    #[validate(length(min = 3, max = 256))]
    pub old_password: String,
    #[validate(length(min = 3, max = 256))]
    pub new_password: String,
    #[validate(must_match(other = "new_password", message = "密码不匹配"))]
    pub repeat_password: String,
}

impl ChangePassword {
    pub fn is_password_matched(&self, target: &str) -> bool {
        verify(&self.old_password, target).unwrap()
    }

    pub async fn change_password(&self, dao: &User) -> Result<User> {
        let mut dao = dao.to_owned();
        let hashed_password = hash(&self.new_password, 4).unwrap();
        dao.password = hashed_password;
        let w = POOL.new_wrapper().eq("id", &dao.id);
        User::update_one(&dao, w).await?;
        Ok(dao)
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ResetPassword {
    #[validate(length(min = 3, max = 256))]
    pub new_password: String,
    #[validate(must_match(other = "new_password", message = "密码不匹配"))]
    pub repeat_password: String,
}

impl ResetPassword {
    pub async fn reset_password(&self, dao: &User) -> Result<User> {
        let mut dao = dao.to_owned();
        let hashed_password = hash(&self.new_password, 4).unwrap();
        dao.password = hashed_password;
        let w = POOL.new_wrapper().eq("id", &dao.id);
        User::update_one(&dao, w).await?;
        Ok(dao)
    }
}
