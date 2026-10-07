pub mod mock;
pub mod traits;

use std::sync::Arc;
use traits::SteamProvider;

pub enum SteamMode {
    Native,
    Mock,
}

pub fn create_provider(mode: SteamMode, app_id: u32) -> Result<Arc<dyn SteamProvider>, String> {
    match mode {
        SteamMode::Native => {
            todo!("Implement steamworks");
        }
        SteamMode::Mock => {
            let provider = mock::MockSteam::new(app_id);
            Ok(Arc::new(provider))
        }
    }
}

//+++ Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_mock_steam_provider() {
        let app_id = 30;
        let provider = create_provider(SteamMode::Mock, app_id);

        assert!(provider.is_ok(), "Should return OK for mock mode");
    }

    #[test]
    #[should_panic(expected = "Implement steamworks")]
    fn should_panic_on_native_steam_provider_todo() {
        let _ = create_provider(SteamMode::Native, 30);
    }
}
//--- Tests
