use crate::sea_orm_models as sea;
use log::{debug, warn};
use sea_orm::entity::*;
use sea_orm::DatabaseConnection;

use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, video_id: i32, tags: Option<Vec<String>>) {
    if let Some(tags) = tags {
        let all = tags.into_iter().map(|tag| sea::videotag::ActiveModel {
            id: NotSet,
            video_id: Set(video_id.try_into().unwrap()),
            tag: Set(Some(tag)),
        });

        let result = sea::videotag::Entity::insert_many(all).exec(db).await;

        match result {
            Ok(_) => debug!("Video tags created"),
            Err(_) => warn!("Error creating video tags"),
        }
    }
}
