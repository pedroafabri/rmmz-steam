use super::STEAM_PROVIDER;
use crate::steam::{SteamMode, create_provider};
use napi_derive::napi;

#[napi]
pub fn init_steam(app_id: u32, is_mock: Option<bool>) -> bool {
    let mode = if is_mock.unwrap_or(false) {
        SteamMode::Mock
    } else {
        SteamMode::Native
    };

    match create_provider(mode, app_id) {
        Ok(provider) => {
            let mut guard = STEAM_PROVIDER.lock().unwrap();
            *guard = Some(provider);
            true
        }
        Err(err) => {
            eprintln!("[N-API Error] {}", err);
            false
        }
    }
}
