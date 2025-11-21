use wayland_client::{Connection, Dispatch, Proxy, QueueHandle, protocol::wl_pointer};
use wayland_protocols::wp::cursor_shape::v1::client::{
    wp_cursor_shape_device_v1::{self, Shape as CursorShape},
    wp_cursor_shape_manager_v1,
};

use crate::wayland::buffer_surface;

impl Dispatch<wl_pointer::WlPointer, ()> for super::State {
    fn event(
        state: &mut Self,
        pointer: &wl_pointer::WlPointer,
        event: wl_pointer::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_pointer::Event::Enter {
                serial, surface, ..
            } => {
                if let Some(cursor_shape_manager) = &state.cursor_shape_manager {
                    let device = wp_cursor_shape_manager_v1::WpCursorShapeManagerV1::get_pointer(
                        &cursor_shape_manager,
                        pointer,
                        qh,
                        (),
                    );
                    wp_cursor_shape_device_v1::WpCursorShapeDeviceV1::set_shape(
                        &device,
                        serial,
                        CursorShape::Crosshair,
                    );
                }
                state.pointer_surface =
                    state.buffer_surfaces.iter().find_map(|(_, bfs)| match bfs {
                        buffer_surface::BufferSurface::Pre(_) => None,
                        buffer_surface::BufferSurface::InProcess(in_process) => {
                            if in_process.base_surface.id() == surface.id() {
                                Some((in_process.monitor_id.clone(), surface.clone()))
                            } else {
                                None
                            }
                        }
                        buffer_surface::BufferSurface::HasOutput(has_output) => {
                            if has_output.base_surface.id() == surface.id() {
                                Some((has_output.monitor_id.clone(), surface.clone()))
                            } else {
                                None
                            }
                        }
                        buffer_surface::BufferSurface::ReadyToDraw(ready_to_draw) => {
                            if ready_to_draw.base_surface.id() == surface.id() {
                                Some((ready_to_draw.monitor_id.clone(), surface.clone()))
                            } else {
                                None
                            }
                        }
                    });
            }
            wl_pointer::Event::Leave { .. } => {
                state.pointer_surface = None;
            }
            wl_pointer::Event::Motion {
                surface_x,
                surface_y,
                ..
            } => {
                state.pointer_position = Some((surface_x, surface_y));
            }
            wl_pointer::Event::Button { .. } => {
                state.running = false;
            }
            _ => (),
        }
    }
}
