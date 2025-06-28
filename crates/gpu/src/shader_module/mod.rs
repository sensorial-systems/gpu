use crate::prelude::*;

mod descriptor;
mod vertex_state;
mod fragment_state;

pub use descriptor::*;
pub use vertex_state::*;
pub use fragment_state::*;

use wgpu::{Device, ShaderModule, ShaderModuleDescriptor};

pub trait ShaderModuleExt {
    fn new<'label>(device: &Device, source: impl IntoExt<ShaderModuleDescriptor<'label>>) -> Self;
}

impl ShaderModuleExt for ShaderModule {
    fn new<'label>(device: &Device, source: impl IntoExt<ShaderModuleDescriptor<'label>>) -> Self {
        device.create_shader_module(source.into_ext())
    }
}
