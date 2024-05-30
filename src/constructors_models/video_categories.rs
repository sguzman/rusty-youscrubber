use crate::sea_orm_models as sea;
use sea_orm::entity::*;
use sea_orm::DatabaseConnection;

use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, video_id: i32, ts: Option<Vec<String>>) {
    if let Some(tags) = ts {
        let all = tags.into_iter().map(|tag| sea::videotag::ActiveModel {
            id: NotSet,
            video_id: Set(video_id.try_into().unwrap()),
            tag: Set(Some(tag)),
        });

        sea::videotag::Entity::insert_many(all)
            .exec(db)
            .await
            .expect("Error creating video tags");
    }
}
