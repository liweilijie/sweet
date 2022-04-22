use crate::error::Result;
use crate::repository::dto::NewUser;
use crate::repository::{dao, dto};
use crate::util::jwt;
use crate::util::jwt::Auth;
use axum::{routing::post, Json, Router};
use serde_json::Value;
use validator::Validate;

async fn register(Json(body): Json<NewUser>) -> Result<Json<Value>> {
    body.validate()?;
    if dao::User::find_by_username(&body.username).await.is_ok() {
        return Err(reject!("用户已存在"));
    }

    let user: dao::User = body.create().await?;
    let token = jwt::generate_token(Auth {
        id: user.id.clone(),
        username: user.username.clone(),
        is_admin: false,
    });
    Ok(reply!({
        "token": token,
        "user": user
    }))
}

async fn login(Json(body): Json<dto::LoginUser>) -> Result<Json<Value>> {
    body.validate()?;
    let user: dao::User = match dao::User::find_by_username_or_email(&body.username_or_email).await
    {
        Ok(user) => user,
        Err(_) => return Err(reject!("用户不存在")),
    };

    if !body.is_password_matched(&user.password) {
        return Err(reject!("密码错误"));
    }

    if user.is_actived == 0 {
        return Err(reject!("用户被禁用"));
    }

    let user: dao::User = body.login(&user).await?;
    let token = jwt::generate_token(Auth {
        id: user.id.clone(),
        username: user.username.clone(),
        is_admin: false,
    });
    Ok(reply!({
        "token": token,
        "user": user
    }))
}

pub fn apply_routes() -> Router {
    let router = Router::new();
    router
        .route("/register", post(register))
        .route("/login", post(login))
}
