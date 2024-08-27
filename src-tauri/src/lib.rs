pub mod models;
pub mod state;

use crate::models::{
    torrent::{Torrent, TorrentStatus},
    TorrentsState,
};
use crate::state::AppState;
use tauri::{async_runtime::Mutex, Manager, State};

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
        .setup(|app| {
            app.manage(Mutex::new(AppState { torrents: initial }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_torrents,
            pause_torrent,
            resume_torrent,
            get_torrent_by_id,
            remove_torrent
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_torrents(state: State<'_, Mutex<AppState>>) -> Result<Vec<Torrent>, ()> {
    Ok(state.lock().await.clone().torrents)
}

#[tauri::command]
async fn pause_torrent(state: State<'_, Mutex<AppState>>, id: u64) -> Result<(), String> {
    if let Some(t) = state.lock().await.torrents.iter_mut().find(|t| t.id == id) {
        t.status = TorrentStatus::Paused;
        Ok(())
    } else {
        Err(format!("No torrent with id {}", id))
    }
}

#[tauri::command]
async fn resume_torrent(state: State<'_, Mutex<AppState>>, id: u64) -> Result<(), String> {
    if let Some(t) = state.lock().await.torrents.iter_mut().find(|t| t.id == id) {
        t.status = TorrentStatus::Downloading;
        Ok(())
    } else {
        Err(format!("No torrent with id {}", id))
    }
}

#[tauri::command]
async fn get_torrent_by_id(state: State<'_, Mutex<AppState>>, id: u64) -> Result<Torrent, ()> {
    let torrents = state.lock().await.clone().torrents;
    Ok(torrents.iter().find(|t| t.id == id).unwrap().clone())
}

#[tauri::command]
async fn remove_torrent(state: State<'_, Mutex<AppState>>, id: u64) -> Result<(), String> {
    let mut app = state.lock().await;
    let len_before = app.torrents.len();
    app.torrents.retain(|t| t.id != id);
    if app.torrents.len() < len_before {
        Ok(())
    } else {
        Err(format!("No torrent found with id {}", id))
    }
}
