use crate::{data, sea_orm_models as sea};
use sea_orm::entity::*;
use sea_orm::{ActiveValue, DatabaseConnection};

use sea_orm::ActiveValue::{NotSet, Set};

fn setff64(option: Option<f32>) -> ActiveValue<Option<f64>> {
    match option {
        Some(value) => Set(Some(value as f64)),
        None => NotSet,
    }
}

pub async fn create(db: &DatabaseConnection, video_id: i32, hso: Option<Vec<data::Fragment>>) {
    if let Some(hs) = hso {
        let all = hs.into_iter().map(|e| sea::fragment::ActiveModel {
            id: NotSet,
            video_id: Set(video_id.try_into().unwrap()),
            duration: setff64(e.duration),
            url: Set(e.url),
        });

        sea::fragment::Entity::insert_many(all)
            .exec(db)
            .await
            .expect("Error creating video tags");
    }
}
