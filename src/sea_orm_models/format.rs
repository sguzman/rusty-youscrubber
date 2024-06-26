//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "format")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(column_type = "Double", nullable)]
    pub abr: Option<f64>,
    pub acodec: Option<String>,
    #[sea_orm(column_type = "Double", nullable)]
    pub aspect_ratio: Option<f64>,
    pub audio_ext: Option<String>,
    pub columns: Option<i32>,
    pub ext: Option<String>,
    pub filesize_approx: Option<i32>,
    pub format: Option<String>,
    pub format_id: Option<String>,
    pub format_note: Option<String>,
    #[sea_orm(column_type = "Double", nullable)]
    pub fps: Option<f64>,
    pub height: Option<i32>,
    pub protocol: Option<String>,
    pub resolution: Option<String>,
    #[sea_orm(column_type = "Double", nullable)]
    pub tbr: Option<f64>,
    pub url: Option<String>,
    pub vbr: Option<i32>,
    pub vcodec: Option<String>,
    pub video_ext: Option<String>,
    pub width: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
