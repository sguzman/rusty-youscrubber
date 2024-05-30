use crate::sea_orm_models::{self as sea};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};

use crate::data::RequestedDownload;

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

fn setbi(option: Option<bool>) -> ActiveValue<Option<i32>> {
    match option {
        Some(value) => Set(Some(value as i32)),
        None => NotSet,
    }
}

pub async fn create(db: &DatabaseConnection, v: Option<Vec<RequestedDownload>>) {
    if let Some(requested_downloads) = v {
        for requested_download in requested_downloads {
            let requested_download = sea::requesteddownload::ActiveModel {
                id: NotSet,
                format_id: Set(requested_download.format_id.unwrap()),

                filesize_approx: setuui(requested_download.filesize_approx),
                format: Set(requested_download.format),
                format_note: Set(requested_download.format_note),
                ext: Set(requested_download.ext),
                acodec: Set(requested_download.acodec),
                vcodec: Set(requested_download.vcodec),
                width: setui(requested_download.width),
                height: setui(requested_download.height),
                resolution: Set(requested_download.resolution),
                fps: setff64(requested_download.fps),
                tbr: setff64(requested_download.tbr),
                abr: setff64(requested_download.abr),
                vbr: setff64(requested_download.vbr),
                audio_ext: Set(requested_download.audio_ext),
                protocol: Set(requested_download.protocol),
                aspect_ratio: setff64(requested_download.aspect_ratio),
                columns: setui(requested_download.columns),
                filename: Set(requested_download.filename),
                write_download_archive: setbi(requested_download.write_download_archive),
            };

            requested_download.insert(db).await.unwrap();
        }
    }
}
