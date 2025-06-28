use crate::prelude::*;
use wgpu::{Extent3d, TextureDescriptor, TextureFormat, TextureUsages, TextureDimension};

pub trait TextureDescriptorExt<'a> {
    fn new(dimension: TextureDimension, format: TextureFormat, size: impl IntoExt<Extent3d>, usage: TextureUsages) -> Self;
    fn with_mip_level_count(self, mip_level_count: u32) -> Self;
    fn with_sample_count(self, sample_count: u32) -> Self;
    fn with_view_formats(self, view_formats: &'a [TextureFormat]) -> Self;
}

impl<'a> TextureDescriptorExt<'a> for TextureDescriptor<'a> {
    fn new(dimension: TextureDimension, format: TextureFormat, size: impl IntoExt<Extent3d>, usage: TextureUsages) -> Self {
        TextureDescriptor {
            label: Default::default(),
            size: size.into_ext(),
            mip_level_count: 1,
            sample_count: 1,
            dimension,
            format,
            usage,
            view_formats: &[],
        }
    }

    fn with_mip_level_count(mut self, mip_level_count: u32) -> Self {
        self.mip_level_count = mip_level_count;
        self
    }

    fn with_sample_count(mut self, sample_count: u32) -> Self {
        self.sample_count = sample_count;
        self
    }

    fn with_view_formats(mut self, view_formats: &'a [TextureFormat]) -> Self {
        self.view_formats = view_formats;
        self
    }
}

crate::impl_label!(TextureDescriptor<'a>);