use crate::json_models::fragment::Fragment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// pub Struct to represent the format field in the json files
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct RequestedFormat {
    pub abr: f32,
    pub acodec: String,
    pub aspect_ratio: f32,
    pub asr: Option<u32>,
    pub audio_channels: Option<u8>,
    pub audio_ext: String,
    pub container: String,
    pub downloader_options: HashMap<String, String>,
    pub dynamic_range: String,
    pub ext: String,
    pub filesize: u64,
    pub filesize_approx: u64,
    pub format: String,
    pub format_id: String,
    pub format_note: String,
    pub fps: f32,
    pub has_drm: bool,
    pub height: u32,
    pub http_headers: HashMap<String, String>,
    pub language: String,
    pub language_preference: i8,
    pub preference: Option<String>,
    pub protocol: String,
    pub quality: f32,
    pub resolution: String,
    pub source_preference: i8,
    pub tbr: f32,
    pub url: String,
    pub vbr: f32,
    pub vcodec: String,
    pub video_ext: String,
    pub width: u32,
}
