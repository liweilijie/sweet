pub mod user;
pub mod category;
pub mod medicinal;

pub use user::Entity as User;
pub use category::Entity as Category;
pub use medicinal::Entity as Medicinal;

#[cfg(test)]
mod tests {
    use bcrypt::hash;
    use chrono::{DateTime, FixedOffset, Local, Utc};
    use rand::distributions::Alphanumeric;
    use rand::Rng;
    use sea_orm::{Database, DatabaseConnection};
    use tracing::info;
    use super::*;
    use sea_orm::{entity::*, error::*, query::*, DbConn, FromQueryResult};
    use uuid::Uuid;


    #[tokio::test]
    async fn user_should_work() -> Result<(), DbErr> {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();


        let db_url = "mysql://root:sbso129129@dev:3306/sweet_dev";
        let db: DatabaseConnection = Database::connect(db_url).await?;

        let users: Vec<user::Model> = User::find().all(&db).await?;

        for user in users.iter() {
            info!("{:?}", user);
        }

        register_user(&db).await?;

        Ok(())
    }

    async fn register_user(db: &DbConn) -> Result<(), DbErr> {
        let id = Uuid::new_v4().to_string();
        let hash_password = hash("asd456123", 4).unwrap();
        let now: DateTime<FixedOffset> = chrono::DateTime::from_utc(Local::now().naive_utc(), FixedOffset::east(8));

        let name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        info!("{}: {:?}", name, now);

        let user = user::ActiveModel {
            id: Set(id),
            username: Set(name.clone()),
            password: Set(hash_password.to_string()),
            email: Set(Some(format!("{}@gmail.com", name))),
            // phone: Set(Some("0987654321".to_string())),
            is_actived: Set(1),
            is_deleted: Set(0),
            is_admin: Set(0),
            // last_logined_at: Set(Some(now)),
            created_at: Set(now),
            ..Default::default()
        };

        let res = User::insert(user).exec(db).await?;

        info!("Inserted: last_insert_id = {}", res.last_insert_id);

        Ok(())
    }
}
