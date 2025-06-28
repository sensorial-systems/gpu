mod descriptor;
pub use descriptor::*;

use wgpu::{Device, Sampler, SamplerDescriptor};

pub trait SamplerExt {
    fn new(device: &Device, descriptor: &SamplerDescriptor) -> Self;
}

impl SamplerExt for Sampler {
    fn new(device: &Device, descriptor: &SamplerDescriptor) -> Self {
        device.create_sampler(descriptor)
    }
}