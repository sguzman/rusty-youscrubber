use crate::json_models::fragment::Fragment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// pub Struct to represent the format field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Format {
    pub abr: f32,
    pub acodec: String,
    pub aspect_ratio: f32,
    pub audio_ext: String,
    pub columns: u32,
    pub ext: String,
    pub filesize_approx: Option<u64>,
    pub format: String,
    pub format_id: String,
    pub format_note: String,
    pub fps: f32,
    pub fragments: Vec<Fragment>,
    pub height: u32,
    pub http_headers: HashMap<String, String>,
    pub protocol: String,
    pub resolution: String,
    pub rows: u32,
    pub tbr: Option<f32>,
    pub url: String,
    pub vbr: f32,
    pub vcodec: String,
    pub video_ext: String,
    pub width: u32,
}
