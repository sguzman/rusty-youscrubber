use crate::sea_orm_models as sea;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, video_id: i32, tags: Option<Vec<String>>) {
    if let Some(tags) = tags {
        for tag in tags {
            let tag = sea::videotag::ActiveModel {
                id: NotSet,
                video_id: Set(video_id.try_into().unwrap()),
                tag: Set(Some(tag)),
            };

            tag.insert(db).await.unwrap();
        }
    }
}
