use crate::steam::traits::*;

pub struct MockSteam {
    pub app_id: u32,
}

impl MockSteam {
    pub fn log(msg: impl AsRef<str>) {
        println!("[MOCKED STEAM] {}", msg.as_ref());
    }
}

// Base trait
impl SteamProvider for MockSteam {
    fn new(app_id: u32) -> Self {
        Self::log(format!("Initialized with AppID {}", app_id));
        Self { app_id }
    }
}

// +++ Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_initialize() {
        let app_id = 30;
        let steam = MockSteam::new(app_id);
        assert_eq!(steam.app_id, app_id, "Should initialize with correct AppID");
    }
}
// --- Tests
