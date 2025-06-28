mod descriptor;
mod entry;
mod layout;

pub use descriptor::*;
pub use entry::*;
pub use layout::*;

use wgpu::{BindGroup, BindGroupDescriptor, Device};

pub trait BindGroupExt<'a> {
    fn new(device: &Device, descriptor: &BindGroupDescriptor<'a>) -> Self;
}

impl<'a> BindGroupExt<'a> for BindGroup {
    fn new(device: &Device, descriptor: &BindGroupDescriptor<'a>) -> Self {
        device.create_bind_group(descriptor)
    }
}