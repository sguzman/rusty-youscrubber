use crate::sea_orm_models as sea;
use sea_orm::entity::*;
use sea_orm::DatabaseConnection;

use crate::data;
use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, video_id: i32, ac_id: i32, acs: Vec<data::Subtitle>) {
    let all = acs.into_iter().map(|ac| sea::subtitle::ActiveModel {
        id: NotSet,
        ext: Set(ac.ext),
        url: Set(ac.url),
        video_id: Set(video_id.try_into().unwrap()),
        name: Set(ac.name),
        subtitle_type_id: Set(ac_id.try_into().unwrap()),
    });

    sea::subtitle::Entity::insert_many(all)
        .exec(db)
        .await
        .expect("Error creating captions");
}
