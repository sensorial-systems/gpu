use crate::prelude::*;

use wgpu::{ShaderModuleDescriptor, ShaderSource};

pub trait ShaderModuleDescriptorExt<'label>: Sized {
    fn new(source: impl IntoExt<Self>) -> Self;
}

impl<'label> ShaderModuleDescriptorExt<'label> for ShaderModuleDescriptor<'label> {
    fn new(source: impl IntoExt<Self>) -> Self {
        source.into_ext()
    }
}

impl<'label> FromExt<ShaderSource<'label>> for ShaderModuleDescriptor<'label> {
    fn from_ext(source: ShaderSource<'label>) -> Self {
        ShaderModuleDescriptor {
            label: Default::default(),
            source
        }
    }
}

crate::impl_label!(ShaderModuleDescriptor<'label>);
