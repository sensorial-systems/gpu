use crate::prelude::*;

mod adapter_options;
pub use adapter_options::*;

use wgpu::{Adapter, Backends, Instance, RequestAdapterOptions};

pub trait AdapterExt: Sized {
    fn new<'surface, 'window: 'surface>(instance: &Instance, options: impl IntoExt<RequestAdapterOptions<'surface, 'window>>) -> impl std::future::Future<Output = Result<Self, wgpu::RequestAdapterError>>;
    fn enumerate(instance: &Instance, backends: Backends) -> Vec<Adapter>;
}

impl AdapterExt for Adapter {
    fn new<'surface, 'window: 'surface>(instance: &Instance, options: impl IntoExt<RequestAdapterOptions<'surface, 'window>>) -> impl std::future::Future<Output = Result<Self, wgpu::RequestAdapterError>> {
        instance.request_adapter(&options.into_ext())
    }

    fn enumerate(instance: &Instance, backends: Backends) -> Vec<Adapter> {
        instance.enumerate_adapters(backends)
    }
}
