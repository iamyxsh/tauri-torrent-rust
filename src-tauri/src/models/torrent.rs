use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Torrent {
    pub id: u64,
    pub name: String,
    pub progress: f64,
    pub downloaded: String,
    pub total: String,
    pub peers: u32,
    pub eta: String,
    pub status: TorrentStatus,
    pub down_speed: Option<String>,
    pub up_speed: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TorrentStatus {
    Downloading,
    Paused,
}
