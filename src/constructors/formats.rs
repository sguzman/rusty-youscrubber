use crate::sea_orm_models as sea;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};

use crate::data;
use sea_orm::ActiveValue::{NotSet, Set};

fn setuui(option: Option<u64>) -> ActiveValue<Option<i32>> {
    match option {
        Some(value) => Set(Some(value as i32)),
        None => NotSet,
    }
}

fn setui(option: Option<u32>) -> ActiveValue<Option<i32>> {
    match option {
        Some(value) => Set(Some(value as i32)),
        None => NotSet,
    }
}

fn setff64(option: Option<f32>) -> ActiveValue<Option<f64>> {
    match option {
        Some(value) => Set(Some(value as f64)),
        None => NotSet,
    }
}

fn setfi(option: Option<f32>) -> ActiveValue<Option<i32>> {
    match option {
        Some(value) => Set(Some(value as i32)),
        None => NotSet,
    }
}

pub async fn create(db: &DatabaseConnection, ts: Option<Vec<data::Format>>) {
    if let Some(tags) = ts {
        for tag in tags {
            let tag = sea::format::ActiveModel {
                id: NotSet,
                url: Set(tag.url),
                width: setui(tag.width),
                height: setui(tag.height),
                abr: setff64(tag.abr),
                acodec: Set(tag.acodec),
                aspect_ratio: setff64(tag.aspect_ratio),
                audio_ext: Set(tag.audio_ext),
                columns: setui(tag.columns),
                ext: Set(tag.ext),
                filesize_approx: setuui(tag.filesize_approx),
                format: Set(tag.format),
                format_id: Set(tag.format_id),
                format_note: Set(tag.format_note),
                fps: setff64(tag.fps),
                protocol: Set(tag.protocol),
                resolution: Set(tag.resolution),
                tbr: setff64(tag.tbr),
                vbr: setfi(tag.vbr),
                vcodec: Set(tag.vcodec),
                video_ext: Set(tag.video_ext),
            };

            tag.insert(db).await.unwrap();
        }
    }
}
