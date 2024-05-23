//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "subtitle")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub video_id: i32,
    pub subtitle_type_id: i32,
    pub ext: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::entry::Entity",
        from = "Column::VideoId",
        to = "super::entry::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Entry,
    #[sea_orm(
        belongs_to = "super::subtitletype::Entity",
        from = "Column::SubtitleTypeId",
        to = "super::subtitletype::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Subtitletype,
}

impl Related<super::entry::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Entry.def()
    }
}

impl Related<super::subtitletype::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subtitletype.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}