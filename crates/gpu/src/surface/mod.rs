mod configuration;

pub use configuration::*;

use wgpu::{Instance, Surface, SurfaceTarget, CreateSurfaceError};

pub trait SurfaceExt<'window>: Sized {
    fn new(instance: &Instance, target: impl Into<SurfaceTarget<'window>>) -> Result<Self, CreateSurfaceError>;
}

impl<'window> SurfaceExt<'window> for Surface<'window> {
    fn new(instance: &Instance, target: impl Into<SurfaceTarget<'window>>) -> Result<Self, CreateSurfaceError> {
        instance.create_surface(target)
    }
}