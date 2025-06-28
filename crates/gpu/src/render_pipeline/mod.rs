mod descriptor;
pub use descriptor::*;

use wgpu::{Device, RenderPipeline, RenderPipelineDescriptor};

pub trait RenderPipelineExt {
    fn new(device: &Device, descriptor: &RenderPipelineDescriptor) -> Self;
}

impl RenderPipelineExt for RenderPipeline {
    fn new(
        device: &Device,
        descriptor: &RenderPipelineDescriptor,
    ) -> Self {
        device.create_render_pipeline(descriptor).into()
    }
}
