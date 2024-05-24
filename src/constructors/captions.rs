use crate::sea_orm_models as sea;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use crate::data;
use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, ac_id: i32, acs: Vec<data::AutomaticCaption>) {
    for ac in acs {
        let tag = sea::caption::ActiveModel {
            id: NotSet,
            auto_cap_id: Set(ac_id.try_into().unwrap()),
            ext: Set(ac.ext),
            url: Set(ac.url),
            protocol: Set(ac.protocol),
        };

        tag.insert(db).await.unwrap();
    }
}
