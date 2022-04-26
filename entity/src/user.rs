use sea_orm::entity::prelude::*;


// 创建 user 表，主键为 id，唯一索引为 username
// user 表与 category, medicinal 表是一对多关系
// CREATE TABLE `user` (
//   `id` varchar(128) NOT NULL,
//   `username` varchar(128) NOT NULL,
//   `password` varchar(256) NOT NULL,
//   `email` varchar(256) NOT NULL DEFAULT '',
//   `phone` varchar(16) NOT NULL DEFAULT '',
//   `is_actived` int(1) NOT NULL DEFAULT '1',
//   `is_deleted` INT(1) NOT NULL DEFAULT '0',
//   `is_admin` INT(1) NOT NULL DEFAULT '0',
//   `last_logined_at` datetime NOT NULL,
//   `created_at` datetime NOT NULL,
//   PRIMARY KEY (`id`),
//   UNIQUE KEY `idx_user_name` (`username`)
// );

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_actived: i32,
    pub is_deleted: i32,
    pub is_admin: i32,
    pub last_logined_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::category::Entity")]
    Category,
    #[sea_orm(has_many = "super::medicinal::Entity")]
    Medicinal,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}
impl Related<super::medicinal::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Medicinal.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}