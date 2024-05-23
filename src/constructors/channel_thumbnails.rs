use crate::{data, sea_orm_models as sea};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};

use sea_orm::ActiveValue::{NotSet, Set};

fn setui(option: Option<u32>) -> ActiveValue<Option<i32>> {
    match option {
        Some(value) => Set(Some(value as i32)),
        None => NotSet,
    }
}

pub async fn create(db: &DatabaseConnection, payload_id: i32, es: Vec<data::ChannelThumbnail>) {
    for e in es {
        let ent = sea::channelthumbnail::ActiveModel {
            id: NotSet,
            channel_id: Set(payload_id.try_into().unwrap()),
            thumbnail_id: Set(e.id),
            url: Set(e.url),
            width: setui(e.width),
            height: setui(e.height),
            resolution: Set(e.resolution),
            preference: Set(e.preference),
        };

        ent.insert(db).await.unwrap();
    }
}
