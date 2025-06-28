mod descriptor;

pub use descriptor::*;

use wgpu::{Adapter, Device, DeviceDescriptor, Queue};


pub trait DeviceExt: Sized {
    fn new<'label>(adapter: &Adapter, options: &DeviceDescriptor<'label>) -> impl std::future::Future<Output = Result<(Self, Queue), wgpu::RequestDeviceError>>;
}

impl DeviceExt for Device {
    fn new<'label>(adapter: &Adapter, options: &DeviceDescriptor<'label>) -> impl std::future::Future<Output = Result<(Self, Queue), wgpu::RequestDeviceError>> {
        adapter.request_device(&options)
    }
}
