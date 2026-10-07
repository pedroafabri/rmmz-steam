// Base Trait
pub trait SteamProvider: Send + Sync {
    fn new(app_id: u32) -> Self
    where
        Self: Sized;
}
