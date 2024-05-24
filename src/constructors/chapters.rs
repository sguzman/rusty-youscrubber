use crate::data::Chapters;
use crate::sea_orm_models as sea;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};

use sea_orm::ActiveValue::{NotSet, Set};

fn setff64(option: Option<f32>) -> ActiveValue<Option<f64>> {
    match option {
        Some(value) => Set(Some(value as f64)),
        None => NotSet,
    }
}

pub async fn create(db: &DatabaseConnection, video_id: i32, cso: Option<Vec<Chapters>>) {
    if let Some(cs) = cso {
        for c in cs {
            let obj = sea::chapter::ActiveModel {
                id: NotSet,
                video_id: Set(video_id.try_into().unwrap()),
                title: Set(c.title),
                start_time: setff64(c.start_time),
                end_time: setff64(c.end_time),
            };

            obj.insert(db).await.unwrap();
        }
    }
}
