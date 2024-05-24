use crate::data::VideoThumbnail;
use crate::sea_orm_models as sea;
use log::{debug, warn};
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use sea_orm::ActiveValue::Set;

pub async fn create(db: &DatabaseConnection, video_id: i32, vtso: Option<Vec<VideoThumbnail>>) {
    if let Some(vts) = vtso {
        for vt in vts {
            let v = sea::videothumbnail::ActiveModel {
                id: Set(video_id),
                preference: Set(vt.preference),
                thumbnail_id: Set(vt.id),
                url: Set(vt.url),
            };

            let result = v.insert(db).await;

            match result {
                Ok(_) => debug!("VideoThumbnail created"),
                Err(e) => {
                    warn!("Error: {}", e);
                }
            }
        }
    }
}
