use crate::sea_orm_models::{self as sea};
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::data::Version;

use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, payload_id: i32, v: Version) {
    let vers = sea::version::ActiveModel {
        id: NotSet,
        channel_id: Set(payload_id.try_into().unwrap()),
        version: Set(v.version),
        current_git_head: Set(v.current_git_head),
        release_git_head: Set(v.release_git_head),
        repository: Set(v.repository),
    };

    vers.insert(db).await.unwrap();
}
