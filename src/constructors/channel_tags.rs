use crate::sea_orm_models as sea;
use sea_orm::entity::*;
use sea_orm::DatabaseConnection;

use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, payload_id: i32, tags: Vec<String>) {
    let all = tags.into_iter().map(|tag| sea::channeltag::ActiveModel {
        id: NotSet,
        channel_id: Set(payload_id.try_into().unwrap()),
        tag: Set(Some(tag)),
    });

    sea::channeltag::Entity::insert_many(all)
        .exec(db)
        .await
        .expect("Error creating video tags");
}
