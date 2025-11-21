mod base_surface_buffer;
mod has_output;
mod in_process;
mod ready_to_draw;
mod types;

pub(super) use base_surface_buffer::BaseSurfaceBuffer;
pub(super) use has_output::HasOutput;
pub(super) use in_process::InProcess;
pub(super) use ready_to_draw::ReadyToDraw;
pub use types::ClientRegion;

//pub(crate) use self::help_template::HelpTemplate;

#[derive(Debug, Clone)]
pub enum BufferSurface {
    Pre(BaseSurfaceBuffer),
    InProcess(InProcess),
    HasOutput(HasOutput),
    ReadyToDraw(ReadyToDraw),
}
