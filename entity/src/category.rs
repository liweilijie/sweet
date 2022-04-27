use sea_orm::entity::prelude::*;

// create category table
// id 与 medicinal 是一对多的关系
// user_id 与 category 是一对多的关系
// CREATE TABLE `category` (
//     `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
//     `name` VARCHAR(500) NOT NULL DEFAULT '',
//     `user_id` VARCHAR(128) NOT NULL,
//     PRIMARY KEY(`id`),
//     UNIQUE KEY `idx_category_name` (`name`, `user_id`),
//     CONSTRAINT `fk-category-users` FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
// );

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    #[sea_orm(unique)]
    pub name: String,
    #[sea_orm(unique)]
    pub user_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::medicinal::Entity")]
    Medicinal,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::medicinal::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Medicinal.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
