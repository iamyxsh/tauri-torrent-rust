use cratetorrent::prelude::*;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::{mpsc, Mutex};

/// A slimmed‐down view of a Torrent’s state for your UI & bookkeeping.
#[derive(Debug, Clone)]
pub struct TorrentInfo {
    pub id: u64,
    pub name: String,
    pub status: TorrentStatus,
    pub progress: f64,
    pub downloaded: u64,
    pub total: u64,
    pub peers: u32,
}

/// Manages the engine and your active torrents.
pub struct TorrentManager {
    engine: Engine<Conf>,
    /// Alerts stream from the engine
    alerts: Mutex<mpsc::UnboundedReceiver<Alert>>,
    /// Your in‐memory map of torrent states
    torrents: Mutex<HashMap<u64, TorrentInfo>>,
}

impl TorrentManager {
    /// Spawn the engine & start your alert loop
    pub async fn new(download_dir: PathBuf) -> anyhow::Result<Arc<Self>> {
        let conf = Conf::new(download_dir);
        let (engine, rx) = engine::spawn(conf)?;
        let manager = Arc::new(Self {
            engine,
            alerts: Mutex::new(rx),
            torrents: Mutex::new(HashMap::new()),
        });

        // fire‐and‐forget the alert listener
        {
            let mgr = manager.clone();
            tokio::spawn(async move {
                let mut alerts = mgr.alerts.lock().await;
                while let Some(alert) = alerts.recv().await {
                    if let Alert::TorrentStats { id, stats } = alert {
                        let mut map = mgr.torrents.lock().await;
                        if let Some(info) = map.get_mut(&id) {
                            info.progress = stats.progress * 100.0;
                            info.downloaded = stats.downloaded;
                            info.total = stats.total;
                            info.peers = stats.num_peers;
                            info.status = if stats.is_seeding {
                                TorrentStatus::Seeding
                            } else {
                                TorrentStatus::Downloading
                            };
                        }
                    }
                }
            });
        }

        Ok(manager)
    }

    /// Add a new torrent via parsed metainfo
    pub async fn add_torrent(&self, metainfo: Metainfo) -> anyhow::Result<u64> {
        let params = TorrentParams {
            metainfo: metainfo.clone(),
            listen_addr: None,
            mode: Mode::Download { seeds: vec![] },
            conf: None,
        };
        let id = self.engine.create_torrent(params)?;
        let mut map = self.torrents.lock().await;
        map.insert(
            id,
            TorrentInfo {
                id,
                name: metainfo.name.clone(),
                status: TorrentStatus::Downloading,
                progress: 0.0,
                downloaded: 0,
                total: metainfo.lengths().iter().sum(),
                peers: 0,
            },
        );
        Ok(id)
    }

    /// Pause an active download
    pub fn pause(&self, id: u64) -> anyhow::Result<()> {
        self.engine.pause(id)?;
        let mut map = futures::executor::block_on(self.torrents.lock());
        if let Some(t) = map.get_mut(&id) {
            t.status = TorrentStatus::Paused;
        }
        Ok(())
    }

    /// Resume a paused download
    pub fn resume(&self, id: u64) -> anyhow::Result<()> {
        self.engine.resume(id)?;
        let mut map = futures::executor::block_on(self.torrents.lock());
        if let Some(t) = map.get_mut(&id) {
            t.status = TorrentStatus::Downloading;
        }
        Ok(())
    }

    /// Remove (and stop) a torrent
    pub fn remove(&self, id: u64) -> anyhow::Result<()> {
        self.engine.remove_torrent(id)?;
        let mut map = futures::executor::block_on(self.torrents.lock());
        map.remove(&id);
        Ok(())
    }

    /// Get a snapshot of all torrents
    pub async fn list(&self) -> Vec<TorrentInfo> {
        self.torrents.lock().await.values().cloned().collect()
    }

    /// Get one torrent by id
    pub async fn get(&self, id: u64) -> Option<TorrentInfo> {
        self.torrents.lock().await.get(&id).cloned()
    }
}
