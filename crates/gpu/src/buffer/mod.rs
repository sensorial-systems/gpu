mod init_descriptor;
pub use init_descriptor::*;

use wgpu::{Device, Buffer};
use wgpu::util::{DeviceExt, BufferInitDescriptor};

pub trait BufferExt {
    fn new(device: &Device, buffer_descriptor: &BufferInitDescriptor) -> Self;
}

impl BufferExt for Buffer {
    fn new(device: &Device, buffer_descriptor: &BufferInitDescriptor) -> Self {
        device.create_buffer_init(buffer_descriptor)
    }
}
