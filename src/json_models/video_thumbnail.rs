use serde::{Deserialize, Serialize};

// pub Struct to represent the thumbnails field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct VideoThumbnail {
    pub id: String,
    pub preference: i32,
    pub url: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub resolution: Option<String>,
}
