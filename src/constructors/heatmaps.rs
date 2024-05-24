use crate::{data, sea_orm_models as sea};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};

use sea_orm::ActiveValue::{NotSet, Set};

fn setff64(option: Option<f32>) -> ActiveValue<Option<f64>> {
    match option {
        Some(value) => Set(Some(value as f64)),
        None => NotSet,
    }
}

pub async fn create(db: &DatabaseConnection, video_id: i32, hso: Option<Vec<data::HeatMap>>) {
    if let Some(hs) = hso {
        for e in hs {
            let ent = sea::heatmap::ActiveModel {
                id: NotSet,
                video_id: Set(video_id.try_into().unwrap()),
                start_time: setff64(e.start_time),
                end_time: setff64(e.end_time),
                value: setff64(e.value),
            };

            ent.insert(db).await.unwrap();
        }
    }
}
