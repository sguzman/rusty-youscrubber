use serde::{Deserialize, Serialize};

// pub Struct to represent the thumbnails field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct ChannelThumbnail {
    pub height: Option<u32>,
    pub id: Option<String>,
    pub resolution: Option<String>,
    pub url: Option<String>,
    pub width: Option<u32>,
    pub preference: Option<i32>,
}
