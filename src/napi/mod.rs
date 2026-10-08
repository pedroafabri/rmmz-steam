pub mod config;

use crate::steam::traits::SteamProvider;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

// O provider global agora é a nossa trait genérica!
pub static STEAM_PROVIDER: Lazy<Mutex<Option<Arc<dyn SteamProvider>>>> =
    Lazy::new(|| Mutex::new(None));
