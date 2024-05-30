use crate::json_models::channel_thumbnail::ChannelThumbnail;
use crate::json_models::files_to_move::FilesToMove;
use crate::json_models::version::Version;
use crate::json_models::video::Video;
use serde::{Deserialize, Serialize};

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
    pub channel: String,
    pub channel_follower_count: Option<u64>,
    pub channel_id: String,
    pub channel_url: String,
    pub description: String,
    pub entries: Vec<Video>,
    pub epoch: u64,
    pub extractor: Option<String>,
    pub extractor_key: Option<String>,
    pub id: String,
    pub modified_date: Option<String>,
    pub original_url: Option<String>,
    pub playlist_count: Option<u64>,
    pub release_year: Option<u32>,
    pub tags: Vec<String>,
    pub thumbnails: Vec<ChannelThumbnail>,
    pub title: String,
    pub uploader: String,
    pub uploader_id: String,
    pub uploader_url: String,
    pub view_count: Option<u64>,
    pub webpage_url: String,
    pub webpage_url_basename: String,
    pub webpage_url_domain: String,
}
