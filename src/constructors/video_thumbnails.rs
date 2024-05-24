use crate::data::VideoThumbnail;
use crate::sea_orm_models as sea;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, video_id: i32, vtso: Option<Vec<VideoThumbnail>>) {
    if let Some(vts) = vtso {
        for vt in vts {
            let v = sea::videothumbnail::ActiveModel {
                id: Set(video_id),
                preference: Set(vt.preference),
                thumbnail_id: NotSet,
                url: Set(vt.url),
            };

            v.insert(db).await.unwrap();
        }
    }
}
