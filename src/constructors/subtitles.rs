use crate::sea_orm_models as sea;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::data;
use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, video_id: i32, ac_id: i32, acs: Vec<data::Subtitle>) {
    for ac in acs {
        let tag = sea::subtitle::ActiveModel {
            id: NotSet,
            ext: Set(ac.ext),
            url: Set(ac.url),
            video_id: Set(video_id.try_into().unwrap()),
            name: Set(ac.name),
            subtitle_type_id: Set(ac_id.try_into().unwrap()),
        };

        tag.insert(db).await.unwrap();
    }
}
