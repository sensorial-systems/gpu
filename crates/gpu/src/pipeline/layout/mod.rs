mod descriptor;
pub use descriptor::*;

use wgpu::{Device, PipelineLayout, PipelineLayoutDescriptor};

pub trait PipelineLayoutExt {
    fn new(device: &Device, descriptor: &PipelineLayoutDescriptor) -> Self;
}

impl PipelineLayoutExt for PipelineLayout {
    fn new(device: &Device, descriptor: &PipelineLayoutDescriptor) -> Self {
        device.create_pipeline_layout(descriptor)
    }
}
