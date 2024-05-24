use crate::sea_orm_models::{self as sea};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};

use crate::data::Video;

use log::debug;
use sea_orm::prelude::DateTime;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::constructors as ctor;

fn setui(option: Option<u32>) -> ActiveValue<Option<i32>> {
    match option {
        Some(value) => Set(Some(value as i32)),
        None => NotSet,
    }
}

fn setuui(option: Option<u64>) -> ActiveValue<Option<i32>> {
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

fn setus(option: Option<u32>) -> ActiveValue<Option<String>> {
    match option {
        Some(value) => Set(Some(value.to_string())),
        None => NotSet,
    }
}

fn setfi(option: Option<f32>) -> ActiveValue<Option<i32>> {
    match option {
        Some(value) => Set(Some(value as i32)),
        None => NotSet,
    }
}

fn num<A, B>(option: Option<A>) -> ActiveValue<Option<B>>
where
    A: Into<B>,
    B: sea_orm::sea_query::Nullable,
    sea_orm::Value: From<B>,
{
    match option {
        Some(value) => Set(Some(value.into())),
        None => NotSet,
    }
}

fn setuus(option: Option<u64>) -> ActiveValue<Option<sea_orm::prelude::DateTime>> {
    match option {
        // Do not warn about deprecated usage
        #[allow(deprecated)]
        Some(value) => Set(Some(DateTime::from_timestamp(value as i64, 0).into())),
        None => NotSet,
    }
}

fn setsd(option: Option<String>) -> ActiveValue<Option<sea_orm::prelude::DateTime>> {
    match option {
        #[allow(deprecated)]
        Some(value) => Set(Some(
            DateTime::from_timestamp(value.parse::<i64>().unwrap(), 0).into(),
        )),
        None => NotSet,
    }
}

pub async fn create(db: &DatabaseConnection, vs: Vec<Video>) {
    for v in vs {
        let ve = sea::entry::ActiveModel {
            id: NotSet,
            channel_id: Set(v.channel_id),
            title: Set(v.title),
            description: Set(v.description),
            duration: setui(v.duration),
            view_count: setuui(v.view_count),
            like_count: setuui(v.like_count),
            comment_count: setuui(v.comment_count),
            last_playlist_index: setui(v.last_playlist_index),
            abr: setff64(v.abr),
            acodec: Set(v.acodec),
            age_limit: setui(v.age_limit),
            aspect_ratio: num(v.aspect_ratio),
            asr: setui(v.asr),
            audio_channels: num(v.audio_channels),
            availability: Set(v.availability),
            average_rating: num(v.average_rating),
            channel: Set(v.channel),
            channel_follower_count: setuui(v.channel_follower_count),
            channel_url: Set(v.channel_url),
            display_id: Set(v.display_id),
            duration_string: setus(v.duration),
            entry_id: Set(v.id),
            epoch: setuus(v.epoch),
            ext: Set(v.ext),
            extractor: Set(v.extractor),
            extractor_key: Set(v.extractor_key),
            filesize: setuui(v.filesize_approx),
            format: Set(v.format),
            format_id: Set(v.format_id),
            format_note: Set(v.format_note),
            fps: setfi(v.fps),
            fulltitle: Set(v.fulltitle),
            has_drm: num(v.has_drm),
            height: setui(v.height),
            is_live: num(v.is_live),
            language: Set(v.language),
            live_status: Set(v.live_status),
            n_entries: setui(v.n_entries),
            original_url: Set(v.original_url),
            playable_in_embed: num(v.playable_in_embed),
            playlist: Set(v.playlist),
            playlist_auto_number: setui(v.playlist_autonumber),
            playlist_count: setuui(v.playlist_count),
            playlist_id: Set(v.playlist_id),
            playlist_index: setui(v.playlist_index),
            playlist_title: Set(v.playlist_title),
            playlist_uploader: Set(v.playlist_uploader),
            playlist_uploader_id: Set(v.playlist_uploader_id),
            protocol: Set(v.protocol),
            release_year: setui(v.release_year),
            release_timestamp: setsd(v.release_date),
            resolution: Set(v.resolution),
            stretched_ratio: setff64(v.stretched_ratio),
            tbr: setff64(v.tbr),
            thumbnail: Set(v.thumbnail),
            uploader_date: setsd(v.upload_date),
            uploader_id: Set(v.uploader_id),
            uploader_url: Set(v.uploader_url),
            vbr: setff64(v.vbr),
            vcodec: Set(v.vcodec),
            width: setui(v.width),
            webpage_url: Set(v.webpage_url),
            webpage_url_basename: Set(v.webpage_url_basename),
            webpage_url_domain: Set(v.webpage_url_domain),
            dynamic_range: num(v.dynamic_range),
        };

        let ve = ve.insert(db).await;

        match ve {
            Ok(vi) => {
                debug!("Record inserted");

                // Format
                //formats(e, d.get("formats"))

                // Heatmaps
                ctor::heatmaps::create(&db, vi.id, v.heatmaps).await;

                // Requested Downloads
                //requested_download(e, d.get("requested_download"))

                // Requested Formats
                //formats(e, d.get("requested_formats"))

                // Subtitles
                //subtitle_type(e, d.get("subtitles"))

                // Video Thumbnails
                //video_thumbnails(e, d.get("thumbnails"))

                // Tags
                ctor::video_tags::create(&db, vi.id, v.tags).await;

                // Format Sort Field
                //format_sort_field(e, d.get("format_sort_field"))

                // Automatic Captions
                //automatic_captions(e, d.get("automatic_captions"))

                // Video Categories
                //video_categories(e, d.get("categories"))

                // Chapters
                //chapters(e, d.get("chapters"))
            }
            Err(e) => {
                debug!("Error: {}", e);
            }
        }
    }
}
