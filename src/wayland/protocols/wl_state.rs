use super::super::buffer_surface::BufferSurface;

use std::collections::{HashMap, HashSet};

use wayland_client::{
    delegate_noop,
    protocol::{wl_buffer, wl_compositor, wl_output, wl_shm, wl_shm_pool, wl_surface},
};
use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_shell_v1;

use wayland_protocols::wp::cursor_shape::v1::client::{
    wp_cursor_shape_device_v1, wp_cursor_shape_manager_v1,
};

// Ignore events from these object types in this example.
delegate_noop!(State: ignore wl_compositor::WlCompositor);
delegate_noop!(State: ignore wl_surface::WlSurface);
delegate_noop!(State: ignore wl_shm::WlShm);
delegate_noop!(State: ignore wl_shm_pool::WlShmPool);
delegate_noop!(State: ignore wl_buffer::WlBuffer);
delegate_noop!(State: ignore zwlr_layer_shell_v1::ZwlrLayerShellV1);
delegate_noop!(State: ignore wp_cursor_shape_manager_v1::WpCursorShapeManagerV1);
delegate_noop!(State: ignore wp_cursor_shape_device_v1::WpCursorShapeDeviceV1);

#[derive(Debug, Default)]
pub struct State {
    pub running: bool,
    pub buffer_surfaces: HashMap<String, BufferSurface>,
    pub cursor_shape_manager: Option<wp_cursor_shape_manager_v1::WpCursorShapeManagerV1>,
    pub layer_shell: Option<zwlr_layer_shell_v1::ZwlrLayerShellV1>,
    pub shm: Option<wl_shm::WlShm>,
    pub compositor: Option<wl_compositor::WlCompositor>,
    pub output_index: u8,
    pub pointer_position: Option<(f64, f64)>,
    pub pointer_surface: Option<(String, wl_surface::WlSurface)>,
}
