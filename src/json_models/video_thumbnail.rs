use serde::{Deserialize, Serialize};

// pub Struct to represent the thumbnails field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct VideoThumbnail {
    pub id: Option<String>,
    pub preference: Option<i32>,
    pub url: Option<String>,
}
