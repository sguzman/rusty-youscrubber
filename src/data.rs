use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Struct to represent the files_to_move field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
struct FilesToMove {}

// Struct to represent the version field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
struct Version {
    pub current_git_head: Option<String>,
    pub release_git_head: Option<String>,
    pub repository: Option<String>,
    pub version: Option<String>,
}

// Struct to represent the automatic_captions field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
struct AutomaticCaption {
    pub ext: Option<String>,
    pub protocol: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

// Struct to represent the entry field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
struct Video {
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
}

// Struct to represent the thumbnails field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
struct ChannelThumbnail {
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
