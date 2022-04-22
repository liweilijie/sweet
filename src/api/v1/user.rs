use crate::error;
use crate::repository::{dao, dto, vo, Dao};
use crate::util::jwt::Auth;
use axum::extract::{Path, Query};
use axum::routing::{get, post, put};
use axum::{Extension, Json, Router};
use serde_json::Value;
use tracing::debug;
use validator::Validate;

async fn all(Query(q): Query<dto::QueryUser>) -> error::Result<Json<Value>> {
    q.validate()?;
    let all = q.find_all().await?;
    Ok(reply!(all))
}

async fn one(Path(id): Path<String>) -> error::Result<Json<Value>> {
    let one: vo::User = match dao::User::find_by_id(&id).await? {
        Some(val) => val.into(),
        None => return Err(reject!("用户不存在")),
    };
    Ok(reply!(one))
}

async fn update(
    Path(id): Path<String>,
    Json(body): Json<dto::UpdateUser>,
) -> error::Result<Json<Value>> {
    debug!("{:?}", body);
    body.validate()?;
    let mut one = match dao::User::find_by_id(&id).await? {
        Some(val) => val,
        None => return Err(reject!("用户不存在")),
    };
    let updated: vo::User = body.save(&mut one).await?.into();
    Ok(reply!(updated))
}

async fn change_password(
    Json(body): Json<dto::ChangePassword>,
    Extension(auth): Extension<Auth>,
) -> error::Result<Json<Value>> {
    body.validate()?;
    let one: dao::User = match dao::User::find_by_id(&auth.id).await? {
        Some(val) => val,
        None => return Err(reject!("用户不存在")),
    };
    if !body.is_password_matched(&one.password) {
        return Err(reject!("旧密码错误"));
    }
    let user: dao::User = body.change_password(&one).await?;
    Ok(reply!(user))
}

async fn reset_password(
    Path(id): Path<String>,
    Json(body): Json<dto::ResetPassword>,
    Extension(auth): Extension<Auth>,
) -> error::Result<Json<Value>> {
    if !auth.is_admin {
        return Err(reject!("仅管理员可访问."));
    }
    body.validate()?;
    let one: dao::User = match dao::User::find_by_id(&id).await? {
        Some(val) => val,
        None => return Err(reject!("用户不存在")),
    };
    let user: dao::User = body.reset_password(&one).await?;
    Ok(reply!(user))
}

async fn me(Extension(auth): Extension<Auth>) -> error::Result<Json<Value>> {
    let one: vo::User = match dao::User::find_by_id(&auth.id).await? {
        Some(val) => val.into(),
        None => return Err(reject!("用户不存在")),
    };
    Ok(reply!(one))
}

pub fn apply_routes() -> Router {
    let router = Router::new();
    router
        .route("/public/user", get(all))
        .route("/public/user/:id", get(one))
        .route("/user/:id", put(update))
        .route("/change/password", post(change_password))
        .route("/reset/:id/password", post(reset_password))
        .route("/me", get(me))
}
