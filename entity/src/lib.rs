pub mod category;
pub mod medicinal;
pub mod user;

pub use category::Entity as Category;
pub use medicinal::Entity as Medicinal;
pub use user::Entity as User;

#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
mod tests {
    use super::*;
    use bcrypt::hash;
    use chrono::{DateTime, Datelike, FixedOffset, Local, TimeZone, Utc};
    use rand::distributions::Alphanumeric;
    use rand::Rng;
    use sea_orm::ActiveValue::NotSet;
    use sea_orm::{entity::*, error::*, query::*, DbConn, FromQueryResult};
    use sea_orm::{Database, DatabaseConnection};
    use tracing::{error, info};
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use uuid::Uuid;

    #[tokio::test]
    async fn user_should_work() -> Result<(), DbErr> {
        // tracing_subscriber::fmt()
        //     .with_max_level(tracing::Level::DEBUG)
        //     .init();
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                "entity=debug,sea_orm=debug".to_string(),
                // std::env::var("RUST_LOG").unwrap_or_else(|_| "strangers=debug".to_string()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();

        let db_url = "mysql://root:sbso129129@dev:3306/sweet_dev";
        let db: DatabaseConnection = Database::connect(db_url).await?;

        // find_all_user(&db).await?;

        // register_user(&db).await?;

        // login(&db).await?;

        // find_by_id(&db).await?;

        // update_user(&db).await?;

        // create_category(&db).await?;

        // create_medicinal(&db).await?;

        // find_by_user_id(&db).await?;

        update_category(&db).await?;

        Ok(())
    }

    async fn find_all_user(db: &DbConn) -> Result<(), DbErr> {
        let users: Vec<user::Model> = User::find().all(db).await?;

        for user in users.iter() {
            info!("{}:{}:{}", user.username, user.password, user.id);
        }

        Ok(())
    }

    async fn register_user(db: &DbConn) -> Result<(), DbErr> {
        let id = Uuid::new_v4().to_string();
        let hash_password = hash("asd456123", 4).unwrap();
        // let now = chrono::DateTime::from_utc(Local::now().naive_utc(), FixedOffset::east(8));
        let hour = 3600;
        let now = FixedOffset::east(8 * hour)
            .from_local_datetime(&Local::now().naive_local())
            .unwrap();

        let name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // let name = "dada".to_string();

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
            last_logined_at: Set(Some(now)),
            created_at: Set(now),
            ..Default::default()
        };

        let res = User::insert(user).exec(db).await?;

        info!("Inserted: last_insert_id = {}", res.last_insert_id);

        Ok(())
    }

    async fn login(db: &DbConn) -> Result<(), DbErr> {
        let username_or_email = "liwei";
        let _passwd = "123456";

        let user = User::find()
            .filter(
                Condition::any()
                    .add(user::Column::Username.eq(username_or_email))
                    .add(user::Column::Email.eq(username_or_email)),
            )
            .one(db)
            .await?;

        info!("found user: {:?}", user);

        Ok(())
    }

    async fn find_by_id(db: &DbConn) -> Result<(), DbErr> {
        let id = "afc67952-c38e-4b3d-a235-63ad02718372";
        let user = User::find_by_id(id.to_owned()).one(db).await?;

        info!("found one user: {:?}", user);

        Ok(())
    }

    #[allow(dead_code)]
    async fn update_user(db: &DbConn) -> Result<(), DbErr> {
        // let pear: Option<fruit::Model> = Fruit::find_by_id(28).one(db).await?;

        // // Into ActiveModel
        // let mut pear: fruit::ActiveModel = pear.unwrap().into();

        // // Update name attribute
        // pear.name = Set("Sweet pear".to_owned());

        // // Update corresponding row in database using primary key value
        // let pear: fruit::Model = pear.update(db).await?;

        let id = "afc67952-c38e-4b3d-a235-63ad02718372";
        let user: Option<user::Model> = User::find_by_id(id.to_owned()).one(db).await?;

        let mut user: user::ActiveModel = user.unwrap().into();
        user.email = Set(Some("453220764@qq.com".to_owned()));
        let user: user::Model = user.update(db).await?;

        info!("updated user: {:?}", user);

        Ok(())
    }

    async fn create_category(db: &DbConn) -> Result<(), DbErr> {
        // use sea_orm::ActiveValue::NotSet;

        // let banana = fruit::ActiveModel {
        //     id: NotSet, // primary key is NotSet
        //     name: Set("Banana".to_owned()),
        //     ..Default::default() // all other attributes are `NotSet`
        // };

        // // Insert, because primary key `id` is `NotSet`
        // let banana: fruit::ActiveModel = banana.save(db).await?;

        // banana.name = Set("Banana Mongo".to_owned());

        // // Update, because primary key `id` is `Unchanged`
        // let banana: fruit::ActiveModel = banana.save(db).await?;

        // let category = "?????????";
        // let user_id = "899b2b59-8f39-484e-9d78-6182de985fdc";
        let category = "??????dada03";
        // let user_id = "afc67952-c38e-4b3d-a235-63ad02718372"; // liwei
        let user_id = "43f2e842-ff52-4ccc-b239-ffb9a642a652"; // dada

        let category = category::ActiveModel {
            id: NotSet,
            name: Set(category.to_owned()),
            user_id: Set(user_id.to_owned()),
        };

        // Insert
        let category: category::ActiveModel = category.save(db).await?;

        info!("inserted category: {:?}", category);

        Ok(())
    }

    async fn is_valid_category(
        db: &DbConn,
        category_id: u32,
        user_id: &str,
    ) -> Result<bool, DbErr> {
        let category = Category::find()
            .filter(
                Condition::all()
                    .add(category::Column::Id.eq(category_id))
                    .add(category::Column::UserId.eq(user_id)),
            )
            .one(db)
            .await?;

        info!("found category: {:?}", category);

        if category.is_none() {
            return Ok(false);
        }

        Ok(true)
    }

    async fn create_medicinal(db: &DbConn) -> Result<(), DbErr> {
        let category_id: u32 = 4;
        let user_id = "afc67952-c38e-4b3d-a235-63ad02718372"; // liwei

        if !is_valid_category(db, category_id, user_id).await? {
            error!("category_id or user_id is valid");
            return Err(DbErr::Custom("category_id is invalid".to_owned()));
        }
        // let user_id = "43f2e842-ff52-4ccc-b239-ffb9a642a652"; // dada
        let now = FixedOffset::east(8 * 3600)
            .from_local_datetime(&Local::now().naive_local())
            .unwrap();
        let date: chrono::NaiveDate = chrono::NaiveDate::from_ymd(
            chrono::Local::now().year(),
            chrono::Local::now().month(),
            chrono::Local::now().day(),
        );

        // #[sea_orm(primary_key, auto_increment = true)]
        // pub id: u32,
        // #[sea_orm(column_name = "category_id")]
        // pub category_id: i32,
        // #[sea_orm(column_name = "name")]
        // pub name: String,
        // #[sea_orm(column_name = "batch_info")]
        // pub batch_info: Option<String>,
        // #[sea_orm(column_name = "spec")]
        // pub spec: Option<String>,
        // #[sea_orm(column_name = "count")]
        // pub count: Option<String>,
        // #[sea_orm(column_name = "validity")]
        // pub validity: NaiveDate,
        // #[sea_orm(column_name = "is_deleted")]
        // pub is_deleted: i32,
        // #[sea_orm(column_name = "created_at")]
        // pub created_at: DateTimeWithTimeZone,
        // #[sea_orm(column_name = "updated_at")]
        // pub updated_at: DateTimeWithTimeZone,
        // #[sea_orm(column_name = "notify_at")]
        // pub notify_at: Option<DateTimeWithTimeZone>,
        // #[sea_orm(column_name = "user_id")]
        // #[sea_orm(primary_key)]
        // pub user_id: String,
        let medicinal = medicinal::ActiveModel {
            category_id: Set(category_id),
            name: Set("??????-???????????????003".to_owned()),
            batch_info: Set(Some("20200205".to_owned())),
            spec: Set(None),
            validity: Set(date),
            user_id: Set(user_id.to_owned()),
            created_at: Set(now),
            updated_at: Set(Some(now)),
            ..Default::default()
        };

        // Insert
        let medicinal: medicinal::ActiveModel = medicinal.save(db).await?;

        info!("inserted medicinal: {:?}", medicinal);

        Ok(())
    }

    async fn find_by_user_id(db: &DbConn) -> Result<(), DbErr> {
        let category_id: u32 = 4;
        let user_id = "afc67952-c38e-4b3d-a235-63ad02718372"; // liwei

        if !is_valid_category(db, category_id, user_id).await? {
            error!("category_id or user_id is valid");
            return Err(DbErr::Custom("category_id is invalid".to_owned()));
        }
        // let user_id = "43f2e842-ff52-4ccc-b239-ffb9a642a652"; // dada

        let medicinals = Medicinal::find()
            .filter(medicinal::Column::UserId.eq(user_id))
            .order_by_asc(medicinal::Column::CreatedAt)
            .all(db)
            .await?;

        for medicinal in medicinals.iter() {
            info!(
                "medicinal: {}=>{}:{}:{}",
                medicinal.category_id, medicinal.name, medicinal.validity, medicinal.created_at
            );
        }

        // related find
        //     let cake_with_fruits: Vec<(cake::Model, Vec<fruit::Model>)> = Cake::find()
        // .find_with_related(Fruit)
        // .all(db)
        // .await?;

        let category_with_medicinal: Vec<(category::Model, Vec<medicinal::Model>)> =
            Category::find()
                .find_with_related(Medicinal)
                .filter(medicinal::Column::UserId.eq(user_id))
                .order_by_asc(medicinal::Column::CreatedAt)
                .all(db)
                .await?;

        for r in category_with_medicinal.iter() {
            info!("category: {:?}", r.0);
            for m in r.1.iter() {
                info!("medicinal: {:?}", m);
            }
        }

        Ok(())
    }

    async fn update_category(db: &DbConn) -> Result<(), DbErr> {
        // let pear: Option<fruit::Model> = Fruit::find_by_id(28).one(db).await?;

        // // Into ActiveModel
        // let mut pear: fruit::ActiveModel = pear.unwrap().into();

        // // Update name attribute
        // pear.name = Set("Sweet pear".to_owned());

        // // Update corresponding row in database using primary key value
        // let pear: fruit::Model = pear.update(db).await?;

        let category: Option<category::Model> = Category::find_by_id(3).one(db).await?;
        let mut category: category::ActiveModel = category.unwrap().into();
        category.name = Set(format!("????????????{}", category.name.as_ref()));

        let category: category::Model = category.update(db).await?;

        Ok(())
    }
}
