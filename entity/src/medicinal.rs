use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use sea_orm::prelude::DateTimeWithTimeZone;

// create medicinal table
// category_id : category 表 与 medicinal 是一对多的关系
// user_id : user  与 medicinal 表是一对多的关系
// CREATE TABLE `medicinal` (
//   `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
//   `category_id` INT NOT NULL,
//   `name` VARCHAR(512) NOT NULL,
//   `batch_info` VARCHAR(512),
//   `spec` VARCHAR(512),
//   `count` VARCHAR(512),
//   `validity` date NOT NULL,
//   `is_deleted` INT(1) NOT NULL DEFAULT '0',
//   `created_at` DATETIME NOT NULL,
//   `updated_at` DATETIME NOT NULL,
//   `notify_at` Timestamp NULL DEFAULT NULL,
//   `user_id` VARCHAR(128) NOT NULL,
//   PRIMARY key(`id`),
//   UNIQUE KEY `idx_medicinal_id` (`id`, `user_id`),
//   UNIQUE KEY `idx_category_medicinal_name` (`user_id`, `category_id`, `name`),
//   CONSTRAINT `fk-medicinal-users` FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
// );

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "medicinal")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: u32,
    #[sea_orm(column_name = "category_id")]
    pub category_id: u32,
    #[sea_orm(column_name = "name")]
    pub name: String,
    #[sea_orm(column_name = "batch_info")]
    pub batch_info: Option<String>,
    #[sea_orm(column_name = "spec")]
    pub spec: Option<String>,
    #[sea_orm(column_name = "count", default_value = "1")]
    pub count: Option<String>,
    #[sea_orm(column_name = "validity")]
    pub validity: NaiveDate,
    #[sea_orm(column_name = "is_deleted", default_value = "0")]
    pub is_deleted: i32,
    #[sea_orm(column_name = "created_at")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(column_name = "updated_at")]
    pub updated_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_name = "notify_at")]
    pub notify_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_name = "user_id")]
    pub user_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
