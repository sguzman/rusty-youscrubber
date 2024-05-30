use crate::json_models2::automatic_caption::AutomaticCaption;
use crate::json_models2::chapters::Chapters;
use crate::json_models2::format::Format;
use crate::json_models2::heatmap::HeatMap;
use crate::json_models2::requested_download::RequestedDownload;
use crate::json_models2::subtitle::Subtitle;
use crate::json_models2::video_thumbnail::VideoThumbnail;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// pub Struct to represent the entry field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Video {
    #[serde(rename = "__last_playlist_index")]
    pub last_playlist_index: Option<u32>,

    #[serde(rename = "_format_sort_fields")]
    pub format_sort_fields: Option<Vec<String>>,
    #[serde(rename = "_has_drm")]
    pub has_drm: Option<bool>,
    pub abr: Option<f32>,
    pub acodec: Option<String>,
    pub age_limit: Option<u32>,
    pub aspect_ratio: Option<f32>,
    pub asr: Option<u32>,
    pub audio_channels: Option<u8>,

    // Map of string to AutomaticCaption
    pub automatic_captions: Option<HashMap<String, Vec<AutomaticCaption>>>,

    pub availability: Option<String>,
    pub average_rating: Option<f32>,
    pub categories: Option<Vec<String>>,
    pub channel: Option<String>,
    pub channel_follower_count: Option<u64>,
    pub channel_id: Option<String>,
    pub channel_url: Option<String>,
    pub chapters: Option<Vec<Chapters>>,
    pub comment_count: Option<u64>,
    pub description: Option<String>,
    pub display_id: Option<String>,
    pub duration: Option<u32>,
    pub dynamic_range: Option<String>,
    pub epoch: Option<u64>,
    pub ext: Option<String>,
    pub extractor: Option<String>,
    pub extractor_key: Option<String>,
    pub filesize_approx: Option<u64>,
    pub format: Option<String>,
    pub format_id: Option<String>,
    pub format_note: Option<String>,
    pub formats: Option<Vec<Format>>,
    pub fps: Option<f32>,
    pub fulltitle: Option<String>,
    pub heatmaps: Option<Vec<HeatMap>>,
    pub height: Option<u32>,
    pub id: Option<String>,
    pub is_live: Option<bool>,
    pub language: Option<String>,
    pub like_count: Option<u64>,
    pub live_status: Option<String>,
    pub n_entries: Option<u32>,
    pub original_url: Option<String>,
    pub playable_in_embed: Option<bool>,
    pub playlist: Option<String>,
    pub playlist_autonumber: Option<u32>,
    pub playlist_count: Option<u64>,
    pub playlist_id: Option<String>,
    pub playlist_index: Option<u32>,
    pub playlist_title: Option<String>,
    pub playlist_uploader: Option<String>,
    pub playlist_uploader_id: Option<String>,
    pub protocol: Option<String>,
    pub release_date: Option<String>,
    pub release_year: Option<u32>,
    pub requested_downloads: Option<Vec<RequestedDownload>>,
    pub requested_formats: Option<Vec<Format>>,
    pub requested_subtitles: Option<Vec<Subtitle>>,
    pub resolution: Option<String>,
    pub stretched_ratio: Option<f32>,
    pub subtitles: Option<HashMap<String, Vec<Subtitle>>>,
    pub tags: Option<Vec<String>>,
    pub tbr: Option<f32>,
    pub thumbnail: Option<String>,
    pub thumbnails: Option<Vec<VideoThumbnail>>,
    pub title: Option<String>,
    pub upload_date: Option<String>,
    pub uploader: Option<String>,
    pub uploader_id: Option<String>,
    pub uploader_url: Option<String>,
    pub vbr: Option<f32>,
    pub vcodec: Option<String>,
    pub view_count: Option<u64>,
    pub was_live: Option<bool>,
    pub webpage_url: Option<String>,
    pub webpage_url_basename: Option<String>,
    pub webpage_url_domain: Option<String>,
    pub width: Option<u32>,
}
