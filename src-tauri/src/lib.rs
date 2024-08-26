pub mod models;

use crate::models::{
    torrent::{Torrent, TorrentStatus},
    TorrentsState,
};
use tauri::State;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let initial: TorrentsState = vec![
        Torrent {
            id: 1,
            name: "Ubuntu ISO".into(),
            progress: 54.3,
            downloaded: "2.17 GB".into(),
            total: "4.00 GB".into(),
            peers: 64,
            eta: "N/A".into(),
            status: TorrentStatus::Paused,
            down_speed: None,
            up_speed: None,
        },
        Torrent {
            id: 2,
            name: "Fedora Live".into(),
            progress: 66.5,
            downloaded: "2.62 GB".into(),
            total: "3.94 GB".into(),
            peers: 111,
            eta: "17s".into(),
            status: TorrentStatus::Downloading,
            down_speed: Some("60.1 MB/s".into()),
            up_speed: None,
        },
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(initial)
        .invoke_handler(tauri::generate_handler![get_torrents])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_torrents(state: State<'_, TorrentsState>) -> Vec<Torrent> {
    state.inner().clone()
}
