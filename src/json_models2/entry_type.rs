use crate::json_models::files_to_move::FilesToMove;
use crate::json_models::video::Video;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct EntyType {
    #[serde(rename = "__files_to_move")]
    pub files_to_move: FilesToMove,
    #[serde(rename = "__x_forwarded_for_ip")]
    pub x_forwarded_for_ip: Option<String>,
    // type_of but in json, its called "_type"
    #[serde(rename = "_type")]
    pub type_of: String,

    pub availability: Option<String>,
    pub channel: Option<String>,
    pub channel_follower_count: Option<u64>,
    pub channel_id: Option<String>,
    pub channel_url: Option<String>,
    pub description: Option<String>,
    pub entries: Vec<Video>,
}
