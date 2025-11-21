use std::{io::Write, os::fd::AsFd};

use crate::wayland::ClientRegion;

use super::super::protocols::State;

use super::base_surface_buffer::BaseSurfaceBuffer;
use wayland_client::{
    QueueHandle,
    protocol::{wl_buffer, wl_compositor, wl_shm, wl_surface},
};

#[derive(Debug, Clone)]
pub struct InProcess {
    pub monitor_id: String,
    pub size: (u16, u16),
    pub buffer: wl_buffer::WlBuffer,
    pub base_surface: wl_surface::WlSurface,
    pub monitor_clients: Vec<ClientRegion>,
}

impl
    From<(
        BaseSurfaceBuffer,
        &wl_shm::WlShm,
        &QueueHandle<State>,
        &wl_compositor::WlCompositor,
    )> for InProcess
{
    fn from(
        (pre, shm, qh, compositor): (
            BaseSurfaceBuffer,
            &wl_shm::WlShm,
            &QueueHandle<State>,
            &wl_compositor::WlCompositor,
        ),
    ) -> Self {
        InProcess {
            monitor_id: pre.monitor_id.clone(),
            size: pre.monitor_size,
            buffer: create_minimal_surface_buffer(&shm, qh),
            base_surface: create_base_surface(compositor, qh),
            monitor_clients: pre.monitor_clients,
        }
    }
}

fn create_minimal_surface_buffer(
    shm: &wl_shm::WlShm,
    qh: &QueueHandle<State>,
) -> wl_buffer::WlBuffer {
    let (init_w, init_h) = (1, 1);

    let file = tempfile::tempfile().unwrap();
    let mut buf = std::io::BufWriter::new(&file);

    for _ in 0..(init_w * init_h) {
        buf.write_all(&[0x00 as u8, 0x00 as u8, 0x00 as u8, 0x00 as u8])
            .unwrap();
    }
    buf.flush().unwrap();

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

fn create_base_surface(
    compositor: &wl_compositor::WlCompositor,
    qh: &QueueHandle<State>,
) -> wl_surface::WlSurface {
    return compositor.create_surface(qh, ());
}
