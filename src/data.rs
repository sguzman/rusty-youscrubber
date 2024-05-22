use serde::{Deserialize, Serialize};

// Struct to represent the files_to_move field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
struct FilesToMove {}

// Struct to represent the version field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
struct Version {
    current_git_head: Option<String>,
    release_git_head: Option<String>,
    repository: Option<String>,
    version: Option<String>,
}

// Struct to represent the entry field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
struct Video {}

// Struct to represent the thumbnails field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
struct ChannelThumbnail {
    height: Option<u32>,
    id: Option<String>,
    resolution: Option<String>,
    url: Option<String>,
    width: Option<u32>,
}

// Structs to represent the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Default)]
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
}
