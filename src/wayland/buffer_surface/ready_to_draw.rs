use crate::wayland::ClientRegion;

use super::super::protocols::State;
use super::has_output::HasOutput;

use std::{
    fs::File,
    io::Write,
    os::fd::AsFd,
    time::{SystemTime, UNIX_EPOCH},
};

use wayland_client::{
    QueueHandle,
    protocol::{
        wl_buffer::{self},
        wl_output, wl_shm, wl_surface,
    },
};
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{self, Layer, ZwlrLayerShellV1},
    zwlr_layer_surface_v1,
};

#[derive(Debug, Clone)]
pub struct ReadyToDraw {
    pub monitor_id: String,
    pub wayland_output: wl_output::WlOutput,
    pub size: (u16, u16),
    pub buffer: wl_buffer::WlBuffer,
    pub base_surface: wl_surface::WlSurface,
    pub wlr_surface: zwlr_layer_surface_v1::ZwlrLayerSurfaceV1,
    pub monitor_clients: Vec<ClientRegion>,
}

impl From<(HasOutput, &zwlr_layer_surface_v1::ZwlrLayerSurfaceV1)> for ReadyToDraw {
    fn from(
        (has_output, layer_surface): (HasOutput, &zwlr_layer_surface_v1::ZwlrLayerSurfaceV1),
    ) -> Self {
        let ready_to_draw = ReadyToDraw {
            monitor_id: has_output.monitor_id,
            size: has_output.size,
            buffer: has_output.buffer,
            base_surface: has_output.base_surface,
            wlr_surface: layer_surface.clone(),
            wayland_output: has_output.wayland_output,
            monitor_clients: has_output.monitor_clients,
        };

        ready_to_draw
            .wlr_surface
            .set_size(ready_to_draw.size.0 as u32, ready_to_draw.size.1 as u32);
        ready_to_draw.base_surface.commit();
        return ready_to_draw;
    }
}

impl ReadyToDraw {
    pub fn acknowledge_configure(self: &Self, serial: u32) {
        self.wlr_surface.ack_configure(serial);
    }

    pub fn actually_draw_buffer_surface(&mut self, shm: &wl_shm::WlShm, qh: &QueueHandle<State>) {
        self.buffer = create_surface_buffer(&shm, qh, self.monitor_clients.clone(), self.size);
        self.base_surface
            .attach(Some(&self.buffer), self.size.0 as i32, self.size.1 as i32);
        self.base_surface.commit();
    }
}

fn create_surface_buffer(
    shm: &wl_shm::WlShm,
    qh: &QueueHandle<State>,
    monitor_clients: Vec<ClientRegion>,
    size: (u16, u16),
) -> wl_buffer::WlBuffer {
    let (init_w, init_h) = size;

    let mut file = tempfile::tempfile().unwrap();

    draw(&mut file, (init_w as i16, init_h as i16), monitor_clients);
    let pool = shm.create_pool(file.as_fd(), init_w as i32 * init_h as i32 * 4, qh, ());
    let buffer = pool.create_buffer(
        0,
        init_w as i32,
        init_h as i32,
        (init_w * 4) as i32,
        wl_shm::Format::Argb8888,
        qh,
        (),
    );
    return buffer.clone();
}
const BG_COLOR: [u8; 4] = [0x00 as u8, 0x00 as u8, 0x00 as u8, 0x00 as u8];
const FG_COLOR: [u8; 4] = [0x40 as u8, 0x40 as u8, 0x40 as u8, 0x2F as u8];

fn is_inside_region((x_cord, y_cord): (i16, i16), client: &ClientRegion) -> bool {
    let x = client.at.0 < x_cord && x_cord < (client.at.0 + client.size.0);
    let y = client.at.1 < y_cord && y_cord < (client.at.1 + client.size.1);

    return x && y;
}

/* fn is_pixel_in_window_bounds((x_cord, y_cord): (i16, i16), client: ClientRegion) -> bool {
    if let Some(client_monitor) = &client.monitor {
        return is_inside_region((x_cord, y_cord), client)
            && client_monitor.to_string() == pointer_monitor_id;
    }
    return false;
}
 */
fn draw(tmp: &mut File, (buf_x, buf_y): (i16, i16), monitor_clients: Vec<ClientRegion>) {
    /*     let start = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("time should go forward"); */
    let mut buf = std::io::BufWriter::new(tmp);
    for y in 0..buf_y {
        for x in 0..buf_x {
            match monitor_clients
                .iter()
                .any(|client| is_inside_region((x, y), client))
            {
                true => buf.write_all(&FG_COLOR).unwrap(),
                false => buf.write_all(&BG_COLOR).unwrap(),
            };
        }
    }
    buf.flush().unwrap();

    /*     let end = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("time should go forward"); */
    //println!("Surface buffer: {}ms", start.abs_diff(end).as_millis());
}
