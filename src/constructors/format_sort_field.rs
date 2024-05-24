use crate::sea_orm_models as sea;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use sea_orm::ActiveValue::{NotSet, Set};

pub async fn create(db: &DatabaseConnection, ts: Option<Vec<String>>) {
    if let Some(tags) = ts {
        for tag in tags {
            let tag = sea::formatsortfield::ActiveModel {
                id: NotSet,
                field: Set(tag),
            };

            tag.insert(db).await.unwrap();
        }
    }
}
