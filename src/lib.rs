pub mod steam;

use napi_derive::napi;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use steamworks::Client;

static STEAM_CLIENT: Lazy<Mutex<Option<Client>>> = Lazy::new(|| Mutex::new(None));

#[napi]
pub fn init_steam(_app_id: u32) -> bool {
    //TODO: Initialize the Steam client
    true
}

#[napi]
pub fn get_persona_name() -> String {
    let guard = STEAM_CLIENT.lock().unwrap();
    if let Some(client) = guard.as_ref() {
        client.friends().name()
    } else {
        "Steam não inicializada".to_string()
    }
}
