// pub Struct to represent the files_to_move field in the json files
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FilesToMove {}

// pub Struct to represent the version field in the json files
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Version {
    pub current_git_head: Option<String>,
    pub release_git_head: Option<String>,
    pub repository: Option<String>,
    pub version: Option<String>,
}

// pub Struct to represent the automatic_captions field in the json files
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AutomaticCaption {
    pub ext: Option<String>,
    pub protocol: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
}

// pub Struct to represent the chapters field in the json files
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Chapters {
    pub start_time: Option<f32>,
    pub end_time: Option<f32>,
    pub title: Option<String>,
}

// pub Struct to represent the fragment field in the json files
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Fragment {
    pub duration: Option<f32>,
    pub url: Option<String>,
}

// pub Struct to represent the format field in the json files
#[derive(Debug, Clone, PartialEq, Default)]
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

// pub Struct to represent the heatmaps field in the json files
#[derive(Debug, Clone, PartialEq, Default)]
pub struct HeatMap {
    pub video_id: Option<String>,
    pub end_time: Option<f32>,
    pub start_time: Option<f32>,
    pub value: Option<f32>,
}

// pub Struct to represent the subtitles field in the json files
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Subtitle {
    pub ext: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

// pub Struct to represent the requested_downloads field in the json files
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RequestedDownload {
    #[serde(rename = "__write_download_archive")]
    pub write_download_archive: Option<bool>,

    pub _filename: Option<String>,
    pub abr: Option<f32>,
    pub acodec: Option<String>,
    pub aspect_ratio: Option<f32>,
    pub asr: Option<u32>,
    pub audio_channels: Option<u8>,
    pub dynamic_range: Option<String>,
    pub ext: Option<String>,
    pub filename: Option<String>,
    pub filesize_approx: Option<u64>,
    pub format: Option<String>,
    pub format_id: Option<String>,
    pub format_note: Option<String>,
    pub fps: Option<f32>,
    pub height: Option<u32>,
    pub language: Option<String>,
    pub protocol: Option<String>,
    pub requested_formats: Option<Vec<Format>>,
    pub resolution: Option<String>,
    pub tbr: Option<f32>,
    pub vbr: Option<f32>,
    pub vcodec: Option<String>,
    pub width: Option<u32>,
}

// pub Struct to represent the entry field in the json files
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Video {
    #[serde(rename = "__last_playlist_index")]
    pub last_playlist_index: Option<u32>,

    #[serde(rename = "_format_sort_fields")]
    pub format_sort_fields: Option<Vec<String>>,
    #[serde(rename = "_has_drm")]
    pub has_drm: Option<bool>,
    pub abr: Option<f32>,
    pub acodec: Option<String>,
    pub age_limit: Option<u32>,
    pub aspect_ratio: Option<f32>,
    pub asr: Option<u32>,
    pub audio_channels: Option<u8>,

    // Map of string to AutomaticCaption
    pub automatic_captions: Option<HashMap<String, Vec<AutomaticCaption>>>,

    pub availability: Option<String>,
    pub average_rating: Option<f32>,
    pub categories: Option<Vec<String>>,
    pub channel: Option<String>,
    pub channel_follower_count: Option<u64>,
    pub channel_id: Option<String>,
    pub channel_url: Option<String>,
    pub chapters: Option<Vec<Chapters>>,
    pub comment_count: Option<u64>,
    pub description: Option<String>,
    pub display_id: Option<String>,
    pub duration: Option<u32>,
    pub epoch: Option<u64>,
    pub ext: Option<String>,
    pub extractor: Option<String>,
    pub extractor_key: Option<String>,
    pub filesize_approx: Option<u64>,
    pub format: Option<String>,
    pub format_id: Option<String>,
    pub format_note: Option<String>,
    pub formats: Option<Vec<Format>>,
    pub fps: Option<f32>,
    pub fulltitle: Option<String>,
    pub heatmaps: Option<Vec<HeatMap>>,
    pub height: Option<u32>,
    pub id: Option<String>,
    pub is_live: Option<bool>,
    pub language: Option<String>,
    pub like_count: Option<u64>,
    pub live_status: Option<String>,
    pub n_entries: Option<u32>,
    pub original_url: Option<String>,
    pub playable_in_embed: Option<bool>,
    pub playlist: Option<String>,
    pub playlist_autonumber: Option<u32>,
    pub playlist_count: Option<u64>,
    pub playlist_id: Option<String>,
    pub playlist_index: Option<u32>,
    pub playlist_title: Option<String>,
    pub playlist_uploader: Option<String>,
    pub playlist_uploader_id: Option<String>,
    pub protocol: Option<String>,
    pub release_date: Option<String>,
    pub release_year: Option<u32>,
    pub requested_downloads: Option<Vec<RequestedDownload>>,
    pub requested_formats: Option<Vec<Format>>,
    pub requested_subtitles: Option<Vec<Subtitle>>,
    pub resolution: Option<String>,
    pub stretched_ratio: Option<f32>,
    pub subtitles: Option<HashMap<String, Vec<Subtitle>>>,
    pub tags: Option<Vec<String>>,
    pub tbr: Option<f32>,
    pub thumbnail: Option<String>,
    pub thumbnails: Option<Vec<VideoThumbnail>>,
    title: Option<String>,
    upload_date: Option<String>,
    uploader: Option<String>,
    uploader_id: Option<String>,
    uploader_url: Option<String>,
    vbr: Option<f32>,
    vcodec: Option<String>,
    view_count: Option<u64>,
    was_live: Option<bool>,
    webpage_url: Option<String>,
    webpage_url_basename: Option<String>,
    webpage_url_domain: Option<String>,
    width: Option<u32>,
}

// pub Struct to represent the thumbnails field in the json files
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ChannelThumbnail {
    pub channel_id: Option<String>,
    pub height: Option<u32>,
    pub id: Option<String>,
    pub resolution: Option<String>,
    pub url: Option<String>,
    pub width: Option<u32>,
}

// pub Struct to represent the thumbnails field in the json files
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VideoThumbnail {
    pub video_id: Option<String>,
    pub id: Option<String>,
    pub preference: Option<i32>,
    pub url: Option<String>,
}