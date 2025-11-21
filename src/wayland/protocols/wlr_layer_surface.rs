use super::super::buffer_surface::{BufferSurface, ReadyToDraw};

use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_surface_v1;

impl Dispatch<zwlr_layer_surface_v1::ZwlrLayerSurfaceV1, Option<String>> for super::State {
    fn event(
        state: &mut Self,
        layer_surface: &zwlr_layer_surface_v1::ZwlrLayerSurfaceV1,
        event: zwlr_layer_surface_v1::Event,
        id: &Option<String>,
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            zwlr_layer_surface_v1::Event::Configure { serial, .. } => {
                if let (Some(id), Some(shm)) = (id, state.shm.clone()) {
                    let buffer_surface = state.buffer_surfaces.get(id);
                    if let Some(buffer_surface) = buffer_surface {
                        let ready_to_draw: Option<ReadyToDraw> = match buffer_surface {
                            BufferSurface::HasOutput(has_output) => {
                                let ready_to_draw =
                                    ReadyToDraw::from((has_output.clone(), layer_surface));
                                ready_to_draw.acknowledge_configure(serial);
                                Some(ready_to_draw)
                            }
                            BufferSurface::ReadyToDraw(bfs) => Some(bfs.clone()),
                            _ => None,
                        };
                        if let Some(mut ready_to_draw) = ready_to_draw {
                            *state.buffer_surfaces.get_mut(id).unwrap() =
                                BufferSurface::ReadyToDraw(ready_to_draw.clone());
                            ready_to_draw.actually_draw_buffer_surface(&shm, qh);
                        }
                    }
                };
            }
            _ => (),
        };
    }
}
