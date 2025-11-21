use wayland_client::protocol::wl_output;

use crate::wayland::ClientRegion;

#[derive(Debug, Clone)]
pub struct BaseSurfaceBuffer {
    pub monitor_id: String,
    pub monitor_size: (u16, u16),
    pub monitor_clients: Vec<ClientRegion>,
}
