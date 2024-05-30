use crate::constructors_models as ctor;
use crate::sea_orm_models as sea;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use sea_orm::ActiveValue::{NotSet, Set};
use std::collections::HashMap;

use crate::data;

pub async fn create(
    db: &DatabaseConnection,
    video_id: i32,
    ato: Option<HashMap<std::string::String, Vec<data::AutomaticCaption>>>,
) {
    if let Some(at) = ato {
        for a in at {
            let aa = sea::automaticcaptions::ActiveModel {
                id: NotSet,
                video_id: Set(video_id.try_into().unwrap()),
                language: Set(a.0),
            };

            let inserted = aa.insert(db).await;

            match inserted {
                Ok(ok) => {
                    ctor::captions::create(&db, ok.id, a.1).await;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
