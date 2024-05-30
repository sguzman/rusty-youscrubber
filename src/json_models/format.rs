use crate::json_models::fragment::Fragment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
