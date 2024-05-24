use crate::data::VideoThumbnail;
use crate::sea_orm_models as sea;
use sea_orm::entity::*;
use sea_orm::DatabaseConnection;

use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, video_id: i32, vtso: Option<Vec<VideoThumbnail>>) {
    if let Some(vts) = vtso {
        let all = vts.into_iter().map(|vt| sea::videothumbnail::ActiveModel {
            id: NotSet,
            video_id: Set(video_id),
            preference: Set(vt.preference),
            thumbnail_id: Set(vt.id),
            url: Set(vt.url),
        });

        sea::videothumbnail::Entity::insert_many(all)
            .exec(db)
            .await
            .expect("Error creating captions");
    }
}
