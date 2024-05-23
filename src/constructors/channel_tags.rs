use crate::sea_orm_models as sea;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, payload_id: i32, tags: Vec<String>) {
    for tag in tags {
        let tag = sea::channeltag::ActiveModel {
            id: NotSet,
            channel_id: Set(payload_id.try_into().unwrap()),
            tag: Set(Some(tag)),
        };

        tag.insert(db).await.unwrap();
    }
}
