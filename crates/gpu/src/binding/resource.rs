use wgpu::{BindingResource, BufferBinding, Sampler, TextureView, Tlas};

use crate::prelude::*;

impl<'a> IntoExt<BindingResource<'a>> for &'a TextureView {
    fn into_ext(self) -> BindingResource<'a> {
        BindingResource::TextureView(self)
    }
}

impl<'a> IntoExt<BindingResource<'a>> for &'a [&'a TextureView] {
    fn into_ext(self) -> BindingResource<'a> {
        BindingResource::TextureViewArray(self)
    }
}

impl<'a> IntoExt<BindingResource<'a>> for &'a Sampler {
    fn into_ext(self) -> BindingResource<'a> {
        BindingResource::Sampler(self)
    }
}

impl<'a> IntoExt<BindingResource<'a>> for BufferBinding<'a> {
    fn into_ext(self) -> BindingResource<'a> {
        BindingResource::Buffer(self)
    }
}

impl<'a> IntoExt<BindingResource<'a>> for &'a [BufferBinding<'a>] {
    fn into_ext(self) -> BindingResource<'a> {
        BindingResource::BufferArray(self)
    }
}

impl<'a> IntoExt<BindingResource<'a>> for &'a [&'a Sampler] {
    fn into_ext(self) -> BindingResource<'a> {
        BindingResource::SamplerArray(self)
    }
}

impl<'a> IntoExt<BindingResource<'a>> for &'a Tlas {
    fn into_ext(self) -> BindingResource<'a> {
        BindingResource::AccelerationStructure(self)
    }
}
