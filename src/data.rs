use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Struct to represent the files_to_move field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
struct FilesToMove {}

// Struct to represent the version field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
struct Version {
    current_git_head: Option<String>,
    release_git_head: Option<String>,
    repository: Option<String>,
    version: Option<String>,
}

// Struct to represent the automatic_captions field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
struct AutomaticCaption {
    ext: Option<String>,
    protocol: Option<String>,
    url: Option<String>,
    name: Option<String>,
}

// Struct to represent the entry field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
struct Video {
    #[serde(rename = "__last_playlist_index")]
    last_playlist_index: Option<u32>,

    #[serde(rename = "_format_sort_fields")]
    format_sort_fields: Option<String>,
    #[serde(rename = "_has_drm")]
    has_drm: Option<bool>,
    abr: Option<f32>,
    acodec: Option<String>,
    age_limit: Option<u32>,
    aspect_ratio: Option<f32>,
    asr: Option<u32>,
    audio_channels: Option<u8>,

    // Map of string to AutomaticCaption
    automatic_captions: Option<HashMap<String, Vec<AutomaticCaption>>>,
}

// Struct to represent the thumbnails field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
struct ChannelThumbnail {
    height: Option<u32>,
    id: Option<String>,
    resolution: Option<String>,
    url: Option<String>,
    width: Option<u32>,
}

// Structs to represent the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Channel {
    #[serde(rename = "__files_to_move")]
    files_to_move: FilesToMove,
    // type_of but in json, its called "_type"
    #[serde(rename = "_type")]
    type_of: String,

    #[serde(rename = "_version")]
    version: Version,

    availability: Option<String>,
    channel: Option<String>,
    channel_follower_count: Option<u64>,
    channel_id: Option<String>,
    channel_url: Option<String>,
    description: Option<String>,
    entries: Vec<Video>,
    epoch: Option<u64>,
    extractor: Option<String>,
    extractor_key: Option<String>,
    id: Option<String>,
    modified_date: Option<String>,
    original_url: Option<String>,
    playlist_count: Option<u64>,
    release_year: Option<u32>,
    tags: Vec<String>,
    thumbnails: Vec<ChannelThumbnail>,
    title: Option<String>,
    uploader: Option<String>,
    uploader_id: Option<String>,
    uploader_url: Option<String>,
    view_count: Option<u64>,
    webpage_url: Option<String>,
    webpage_url_basename: Option<String>,
    webpage_url_domain: Option<String>,
}
