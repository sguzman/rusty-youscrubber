//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "channelthumbnail")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub channel_id: i32,
    pub thumbnail_id: Option<String>,
    pub preference: Option<i32>,
    pub resolution: Option<String>,
    pub url: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::payload::Entity",
        from = "Column::ChannelId",
        to = "super::payload::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Payload,
}

impl Related<super::payload::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payload.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
