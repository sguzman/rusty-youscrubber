use crate::constructors_models as ctor;
use crate::sea_orm_models as sea;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

use sea_orm::ActiveValue::{NotSet, Set};
use std::collections::HashMap;

use crate::data;

pub async fn create(
    db: &DatabaseConnection,
    video_id: i32,
    ato: Option<HashMap<std::string::String, Vec<data::Subtitle>>>,
) {
    if let Some(at) = ato {
        for a in at {
            let aa = sea::subtitletype::ActiveModel {
                id: NotSet,
                video_id: Set(video_id.try_into().unwrap()),
                language: Set(Some(a.0)),
            };

            let inserted = aa.insert(db).await;

            match inserted {
                Ok(ok) => {
                    ctor::subtitles::create(&db, video_id, ok.id, a.1).await;
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
