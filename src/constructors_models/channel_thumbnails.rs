use crate::{data, sea_orm_models as sea};
use log::{debug, warn};
use sea_orm::entity::*;
use sea_orm::{ActiveValue, DatabaseConnection};

use sea_orm::ActiveValue::{NotSet, Set};

fn setui(option: Option<u32>) -> ActiveValue<Option<i32>> {
    match option {
        Some(value) => Set(Some(value as i32)),
        None => NotSet,
    }
}

pub async fn create(db: &DatabaseConnection, payload_id: i32, es: Vec<data::ChannelThumbnail>) {
    for e in es {
        let c = sea::channelthumbnail::ActiveModel {
            id: NotSet,
            channel_id: Set(payload_id.try_into().unwrap()),
            thumbnail_id: Set(e.id),
            url: Set(e.url),
            width: setui(e.width),
            height: setui(e.height),
            resolution: Set(e.resolution),
            preference: Set(e.preference),
        };

        let inserted = c.insert(db).await;

        match inserted {
            Ok(_) => debug!("Inserted channel thumbnail"),
            Err(_) => warn!("Error inserting channel thumbnail"),
        }
    }
}
