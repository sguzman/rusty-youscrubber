use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// pub Struct to represent the files_to_move field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct FilesToMove {}

// pub Struct to represent the version field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Version {
    pub current_git_head: Option<String>,
    pub release_git_head: Option<String>,
    pub repository: Option<String>,
    pub version: Option<String>,
}

// pub Struct to represent the automatic_captions field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct AutomaticCaption {
    pub ext: Option<String>,
    pub protocol: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

// pub Struct to represent the chapters field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Chapters {
    pub start_time: Option<f32>,
    pub end_time: Option<f32>,
    pub title: Option<String>,
}

// pub Struct to represent the fragment field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Fragment {
    pub duration: Option<f32>,
    pub url: Option<String>,
}

// pub Struct to represent the format field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Format {
    pub abr: Option<f32>,
    pub acodec: Option<String>,
    pub aspect_ratio: Option<f32>,
    pub audio_ext: Option<String>,
    pub columns: Option<u32>,
    pub ext: Option<String>,
    pub filesize_approx: Option<u64>,
    pub format: Option<String>,
    pub format_id: Option<String>,
    pub format_note: Option<String>,
    pub fps: Option<f32>,
    pub fragments: Option<Vec<Fragment>>,
    pub height: Option<u32>,
    pub http_headers: Option<HashMap<String, String>>,
    pub protocol: Option<String>,
    pub resolution: Option<String>,
    pub rows: Option<u32>,
    pub tbr: Option<f32>,
    pub url: Option<String>,
    pub vbr: Option<f32>,
    pub vcodec: Option<String>,
    pub video_ext: Option<String>,
    pub width: Option<u32>,
}

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

    availability: Option<String>,
    average_rating: Option<f32>,
    categories: Option<Vec<String>>,
    channel: Option<String>,
    channel_follower_count: Option<u64>,
    channel_id: Option<String>,
    channel_url: Option<String>,
    chapters: Option<Vec<Chapters>>,
    comment_count: Option<u64>,
    description: Option<String>,
    display_id: Option<String>,
    duration: Option<u32>,
    epoch: Option<u64>,
    ext: Option<String>,
    extractor: Option<String>,
    extractor_key: Option<String>,
    filesize_approx: Option<u64>,
    format: Option<String>,
    format_id: Option<String>,
    format_note: Option<String>,
    formats: Option<Vec<Format>>,
}

// pub Struct to represent the thumbnails field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct ChannelThumbnail {
    pub height: Option<u32>,
    pub id: Option<String>,
    pub resolution: Option<String>,
    pub url: Option<String>,
    pub width: Option<u32>,
}

// Structs to represent the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Channel {
    #[serde(rename = "__files_to_move")]
    pub files_to_move: FilesToMove,
    // type_of but in json, its called "_type"
    #[serde(rename = "_type")]
    pub type_of: String,

    #[serde(rename = "_version")]
    pub version: Version,

    pub availability: Option<String>,
    pub channel: Option<String>,
    pub channel_follower_count: Option<u64>,
    pub channel_id: Option<String>,
    pub channel_url: Option<String>,
    pub description: Option<String>,
    pub entries: Vec<Video>,
    pub epoch: Option<u64>,
    pub extractor: Option<String>,
    pub extractor_key: Option<String>,
    pub id: Option<String>,
    pub modified_date: Option<String>,
    pub original_url: Option<String>,
    pub playlist_count: Option<u64>,
    pub release_year: Option<u32>,
    pub tags: Vec<String>,
    pub thumbnails: Vec<ChannelThumbnail>,
    pub title: Option<String>,
    pub uploader: Option<String>,
    pub uploader_id: Option<String>,
    pub uploader_url: Option<String>,
    pub view_count: Option<u64>,
    pub webpage_url: Option<String>,
    pub webpage_url_basename: Option<String>,
    pub webpage_url_domain: Option<String>,
}
