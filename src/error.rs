use thiserror::Error;
use wgpu::{CreateSurfaceError, RequestDeviceError};
use winit::error::OsError;

#[derive(Debug, Error)]
pub enum MageError {
    #[error("unable to open window")]
    WindowError(#[from] OsError),

    #[error("unable to create rendering surface")]
    CreateSurfaceError(#[from] CreateSurfaceError),

    #[error("unable to create GPU adapter")]
    BadAdapter,

    #[error("unable to create GPU device")]
    BadDevice(#[from] RequestDeviceError),
}
