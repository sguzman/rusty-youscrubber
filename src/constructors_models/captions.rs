use crate::sea_orm_models::{self as sea};
use sea_orm::entity::*;
use sea_orm::DatabaseConnection;

use crate::data;
use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, ac_id: i32, acs: Vec<data::AutomaticCaption>) {
    let all = acs.into_iter().map(|ac| sea::caption::ActiveModel {
        id: NotSet,
        auto_cap_id: Set(ac_id.try_into().unwrap()),
        ext: Set(ac.ext),
        url: Set(ac.url),
        protocol: Set(ac.protocol),
    });

    sea::caption::Entity::insert_many(all)
        .exec(db)
        .await
        .expect("Error creating captions");
}
