use crate::models::TorrentsState;

#[derive(Default, Clone)]
pub struct AppState {
    pub torrents: TorrentsState,
}
