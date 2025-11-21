use crate::wayland::buffer_surface::ClientRegion;

use super::buffer_surface::{BaseSurfaceBuffer, BufferSurface, HasOutput, InProcess};
use super::protocols::State;

use hyprland::data::{Client as HyClient, Clients as HyClients, Monitors as HyMonitors};
use hyprland::shared::WorkspaceId;
use wayland_client::EventQueue;

use std::collections::HashMap;

use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::{wl_compositor, wl_keyboard, wl_output, wl_pointer, wl_registry, wl_seat, wl_shm},
};
use wayland_protocols::wp::cursor_shape::v1::client::{
    wp_cursor_shape_device_v1, wp_cursor_shape_manager_v1,
};
use wayland_protocols_wlr::layer_shell::v1::client::{zwlr_layer_shell_v1, zwlr_layer_surface_v1};

impl Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name, interface, ..
        } = event
        {
            match &interface[..] {
                "wl_compositor" => {
                    let compositor =
                        registry.bind::<wl_compositor::WlCompositor, _, _>(name, 1, qh, ());
                    state.compositor = Some(compositor);
                }
                "wl_shm" => {
                    let shm = registry.bind::<wl_shm::WlShm, _, _>(name, 1, qh, ());
                    if let Some(compositor) = &state.compositor {
                        state.buffer_surfaces.iter_mut().for_each(|(_, bfs)| {
                            match bfs {
                                BufferSurface::Pre(pre) => {
                                    let update =
                                        InProcess::from((pre.clone(), &shm, qh, compositor));
                                    *bfs = BufferSurface::InProcess(update);
                                }
                                _ => (),
                            };
                        });
                    }

                    state.shm = Some(shm);
                }
                "wl_seat" => {
                    registry.bind::<wl_seat::WlSeat, _, _>(name, 1, qh, ());
                }
                "wl_pointer" => {
                    registry.bind::<wl_pointer::WlPointer, _, _>(name, 1, qh, ());
                }
                "wl_keyboard" => {
                    registry.bind::<wl_keyboard::WlKeyboard, _, _>(name, 1, qh, ());
                }
                "wp_cursor_shape_manager_v1" => {
                    let manager = registry
                        .bind::<wp_cursor_shape_manager_v1::WpCursorShapeManagerV1, _, _>(
                            name,
                            1,
                            qh,
                            (),
                        );
                    state.cursor_shape_manager = Some(manager)
                }
                "wp_cursor_shape_device_v1" => {
                    registry.bind::<wp_cursor_shape_device_v1::WpCursorShapeDeviceV1, _, _>(
                        name,
                        1,
                        qh,
                        (),
                    );
                }
                "wl_output" => {
                    let output = registry.bind::<wl_output::WlOutput, _, _>(name, 1, qh, ());
                    if let Some(layer_shell) = state.layer_shell.as_ref() {
                        /* why did they make it so the output object doesn't have an ID on its own!?!?!?!
                            I could get the ID from an event but wl_output events only fire once you've
                            attached a surface to the output so im forced to attach stuff here and guess
                            which display it is.
                            If the is language had WeakMap i could at least us that to match outputs in the event
                            that gets fired after initially attaching but oh well \(;-;)/
                        */
                        if let Some(buffer_surface) =
                            state.buffer_surfaces.get(&state.output_index.to_string())
                        {
                            match buffer_surface {
                                BufferSurface::InProcess(in_process) => {
                                    let in_process = in_process.clone();

                                    let has_output =
                                        HasOutput::from((in_process, layer_shell, &output, qh));
                                    *state
                                        .buffer_surfaces
                                        .get_mut(&state.output_index.to_string())
                                        .unwrap() = BufferSurface::HasOutput(has_output);
                                    state.output_index += 1;
                                }
                                _ => (),
                            }
                        }
                    }
                }
                "zwlr_layer_shell_v1" => {
                    let zwlr_layer = registry.bind::<zwlr_layer_shell_v1::ZwlrLayerShellV1, _, _>(
                        name,
                        1,
                        qh,
                        (),
                    );
                    state.layer_shell = Some(zwlr_layer);
                }
                "zwlr_layer_surface_v1" => {
                    registry
                        .bind::<zwlr_layer_surface_v1::ZwlrLayerSurfaceV1, _, _>(name, 1, qh, None);
                }
                _ => {}
            }
        }
    }
}
/*
fn cords_relative_to_surface((x, y): (i16, i16)) -> (i16, i16) {}

fn is_inside_region((x_cord, y_cord): (i16, i16), client: ClientRegion) -> bool {
    let x = client.at.0 < x_cord && x_cord < (client.at.0 + client.size.0);
    let y = client.at.1 < y_cord && y_cord < (client.at.1 + client.size.1);

    return x && y;
}

fn is_pixel_in_window_bounds((x_cord, y_cord): (i16, i16), client: ClientRegion) -> bool {
    if let Some(client_monitor) = &client.monitor {
        return is_inside_region((x_cord, y_cord), client)
            && client_monitor.to_string() == pointer_monitor_id;
    }
    return false;
} */

pub fn create_state_and_region_bounds<'c>(
    clients: &'c HyClients,
    monitors: &HyMonitors,
) -> (State, Vec<ClientRegion>) {
    let active_workspaces_ids: Vec<WorkspaceId> = monitors
        .iter()
        .map(|monitor| monitor.active_workspace.id)
        .collect();

    let clients: Vec<HyClient> = clients
        .iter()
        .filter_map(|client| {
            if client.mapped && active_workspaces_ids.contains(&client.workspace.id) {
                return Some(client.clone());
            }
            return None;
        })
        .collect();

    let client_regions = clients.iter().map(|client| {
        if let Some(client_monitor_id) = client.monitor {
            if let Some(monitor) = monitors
                .iter()
                .find(|monitor| monitor.id == client_monitor_id)
            {
                let relative_x = (client.at.0 as i32) - monitor.x;
                let relative_y = (client.at.1 as i32) - monitor.y;
                return ClientRegion {
                    at: (relative_x as i16, relative_y as i16),
                    size: client.size.clone(),
                    monitor: Some(client_monitor_id.to_string()),
                    client_id: client.address.clone(),
                };
            }
        }
        return ClientRegion {
            at: client.at.clone(),
            size: client.size.clone(),
            monitor: None,
            client_id: client.address.clone(),
        };
    });

    let buffer_surfaces = HashMap::from_iter(monitors.iter().map(|monitor| {
        let monitor_clients: Vec<ClientRegion> = client_regions
            .clone()
            .filter(|client| match client.monitor.clone() {
                Some(client_monitor) => monitor.id.to_string() == client_monitor,
                None => false,
            })
            .collect();

        return (
            monitor.id.to_string(),
            BufferSurface::Pre(BaseSurfaceBuffer {
                monitor_id: monitor.id.to_string(),
                monitor_size: (monitor.width, monitor.height),
                monitor_clients: monitor_clients.clone(),
            }),
        );
    }));

    return (
        State {
            running: true,
            buffer_surfaces,
            cursor_shape_manager: None,
            layer_shell: None,
            shm: None,
            compositor: None,
            // this sucks but so do i, it seems like output ids start at 0 and count up
            output_index: 0,
            pointer_position: None,
            pointer_surface: None,
        },
        client_regions.collect(),
    );
}

pub fn create_wayland_window_select() -> EventQueue<State> {
    let conn = Connection::connect_to_env().unwrap();

    let event_queue = conn.new_event_queue();
    let qhandle = event_queue.handle();

    let display = conn.display();
    display.get_registry(&qhandle, ());

    return event_queue;
}
