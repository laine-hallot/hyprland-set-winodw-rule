use hyprland::shared::Address;

#[derive(Debug, Clone)]
pub struct ClientRegion {
    pub at: (i16, i16),
    pub size: (i16, i16),
    pub monitor: Option<String>,
    pub client_id: Address,
}
