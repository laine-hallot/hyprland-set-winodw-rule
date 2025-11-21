use crate::wayland::ClientRegion;

use super::super::protocols::State;
use super::in_process::InProcess;
use wayland_client::{
    QueueHandle,
    protocol::{
        wl_buffer::{self},
        wl_output, wl_surface,
    },
};
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{self, Layer, ZwlrLayerShellV1},
    zwlr_layer_surface_v1,
};

#[derive(Debug, Clone)]
pub struct HasOutput {
    pub monitor_id: String,
    pub wayland_output: wl_output::WlOutput,
    pub size: (u16, u16),
    pub buffer: wl_buffer::WlBuffer,
    pub base_surface: wl_surface::WlSurface,
    pub wlr_surface: zwlr_layer_surface_v1::ZwlrLayerSurfaceV1,
    pub monitor_clients: Vec<ClientRegion>,
}
// the DX on using a tuple to include the necessary extra data to go from InProcess to HasOutput is suboptimal
// but I don't think theres a better way to do this while still using From
impl
    From<(
        super::in_process::InProcess,
        &zwlr_layer_shell_v1::ZwlrLayerShellV1,
        &wl_output::WlOutput,
        &QueueHandle<State>,
    )> for HasOutput
{
    fn from(
        (in_process, layer_shell, output, qh): (
            InProcess,
            &zwlr_layer_shell_v1::ZwlrLayerShellV1,
            &wl_output::WlOutput,
            &QueueHandle<State>,
        ),
    ) -> Self {
        let wlr_surface = create_layer_surface(
            layer_shell,
            output,
            qh,
            &(in_process.base_surface).clone(),
            in_process.monitor_id.clone(),
        );
        let has_output = HasOutput {
            monitor_id: in_process.monitor_id,
            size: in_process.size,
            buffer: in_process.buffer,
            base_surface: in_process.base_surface,
            wlr_surface: wlr_surface,
            wayland_output: output.clone(),
            monitor_clients: in_process.monitor_clients,
        };
        has_output.init_wlr_surface();
        return has_output;
    }
}

impl HasOutput {
    pub fn init_wlr_surface(&self) {
        self.wlr_surface
            .set_size(self.size.0 as u32, self.size.1 as u32);
        self.wlr_surface
            .set_anchor(zwlr_layer_surface_v1::Anchor::Top);

        self.base_surface.commit();
    }
}

fn create_layer_surface(
    zwlr_layer: &ZwlrLayerShellV1,
    output: &wl_output::WlOutput,
    qh: &QueueHandle<State>,
    base_surface: &wl_surface::WlSurface,
    monitor_id: String,
) -> zwlr_layer_surface_v1::ZwlrLayerSurfaceV1 {
    return zwlr_layer.get_layer_surface(
        base_surface,
        Some(output),
        Layer::Top,
        "selection".to_string(),
        qh,
        Some(monitor_id),
    );
}
