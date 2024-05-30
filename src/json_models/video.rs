use crate::json_models::automatic_caption::AutomaticCaption;
use crate::json_models::chapter::Chapters;
use crate::json_models::format::Format;
use crate::json_models::heatmap::HeatMap;
use crate::json_models::requested_download::RequestedDownload;
use crate::json_models::subtitle::Subtitle;
use crate::json_models::video_thumbnail::VideoThumbnail;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::requested_format::RequestedFormat;

// pub Struct to represent the entry field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Video {
    #[serde(rename = "__last_playlist_index")]
    pub last_playlist_index: u32,

    #[serde(rename = "_format_sort_fields")]
    pub format_sort_fields: Vec<String>,
    #[serde(rename = "_has_drm")]
    pub has_drm: Option<bool>,
    pub abr: f32,
    pub acodec: String,
    pub age_limit: u32,
    pub aspect_ratio: f32,
    pub asr: u32,
    pub audio_channels: u8,

    // Map of string to AutomaticCaption
    pub automatic_captions: HashMap<String, Vec<AutomaticCaption>>,

    pub availability: String,
    pub average_rating: Option<f32>,
    pub categories: Vec<String>,
    pub channel: String,
    pub channel_follower_count: u64,
    pub channel_id: String,
    pub channel_url: String,
    pub chapters: Option<Vec<Chapters>>,
    pub comment_count: u64,
    pub description: String,
    pub display_id: String,
    pub duration: u32,
    pub dynamic_range: String,
    pub epoch: u64,
    pub ext: String,
    pub extractor: String,
    pub extractor_key: String,
    pub filesize_approx: u64,
    pub format: String,
    pub format_id: String,
    pub format_note: String,
    pub formats: Vec<Format>,
    pub fps: f32,
    pub fulltitle: String,
    pub heatmaps: Option<Vec<HeatMap>>,
    pub height: u32,
    pub id: String,
    pub is_live: bool,
    pub language: String,
    pub like_count: u64,
    pub live_status: String,
    pub n_entries: u32,
    pub original_url: String,
    pub playable_in_embed: bool,
    pub playlist: String,
    pub playlist_autonumber: u32,
    pub playlist_count: u64,
    pub playlist_id: String,
    pub playlist_index: u32,
    pub playlist_title: String,
    pub playlist_uploader: String,
    pub playlist_uploader_id: String,
    pub protocol: String,
    pub release_timestamp: Option<String>,
    pub release_year: Option<u32>,
    pub requested_downloads: Vec<RequestedDownload>,
    pub requested_formats: Option<Vec<RequestedFormat>>,
    pub requested_subtitles: Option<Vec<Subtitle>>,
    pub resolution: String,
    pub stretched_ratio: Option<f32>,
    pub subtitles: HashMap<String, Vec<Subtitle>>,
    pub tags: Vec<String>,
    pub tbr: f32,
    pub thumbnail: String,
    pub thumbnails: Vec<VideoThumbnail>,
    pub title: String,
    pub upload_date: String,
    pub uploader: String,
    pub uploader_id: String,
    pub uploader_url: String,
    pub vbr: f32,
    pub vcodec: String,
    pub view_count: u64,
    pub was_live: bool,
    pub webpage_url: String,
    pub webpage_url_basename: String,
    pub webpage_url_domain: String,
    pub width: u32,
}
