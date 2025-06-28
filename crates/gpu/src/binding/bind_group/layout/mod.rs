mod descriptor;
mod entry;

pub use descriptor::*;
pub use entry::*;
use wgpu::{BindGroupLayout, BindGroupLayoutDescriptor, Device};

pub trait BindGroupLayoutExt {
    fn new(device: &Device, descriptor: &BindGroupLayoutDescriptor) -> Self;
}

impl<'a> BindGroupLayoutExt for BindGroupLayout {
    fn new(device: &Device, descriptor: &BindGroupLayoutDescriptor) -> Self {
        device.create_bind_group_layout(descriptor)
    }
}